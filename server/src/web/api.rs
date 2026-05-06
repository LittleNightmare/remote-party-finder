use std::{collections::HashSet, convert::Infallible, sync::Arc};

use chrono::Utc;
use mongodb::bson::{doc, to_bson};
use serde::{Serialize, Deserialize};
use tokio_stream::StreamExt;
use warp::{
    Filter,
    filters::BoxedFilter,
    http::StatusCode,
    Reply,
};

use crate::{
    ffxiv::Language, listing::{DutyCategory, JobFlags}, listing_container::QueriedListing, sestring_ext::SeStringExt, web::State
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    data: T,
    pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    total: usize,
    page: usize,
    per_page: usize,
    total_pages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiListing {
    id: u64,
    name: String,
    description: String,
    created_world: String,
    created_world_id: u32,
    home_world: String,
    home_world_id: u32,
    category: String,
    category_id: u32,
    duty: String,
    min_item_level: u16,
    slots_filled: usize,
    slots_available: u8,
    time_left: f64,
    updated_at: String,
    is_cross_world: bool,
    datacenter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedApiListing {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub created_world: String,
    pub home_world: String,
    pub category: String,
    pub duty: String,
    pub min_item_level: u16,
    pub slots_filled: usize,
    pub slots_available: u8,
    pub time_left: f64,
    pub updated_at: String,
    pub is_cross_world: bool,
    // 添加更多详细信息
    pub beginners_welcome: bool,
    pub duty_type: String,
    pub objective: String,
    pub conditions: String,
    pub loot_rules: String,
    pub slots: Vec<SlotInfo>,
    pub datacenter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
    pub filled: bool,
    pub role: Option<String>,
    pub role_id: u32,
    pub job: Option<String>,
    pub job_id: Vec<u32>,
}

fn accepted_slot_bits_for_job_ids(job_ids: &[u32]) -> Vec<u64> {
    job_ids
        .iter()
        .filter_map(|&job_id| JobFlags::accepted_slot_bit_for_job_id(job_id))
        .collect()
}

fn canonical_job_id_for_code(code: &str) -> Option<u32> {
    let accepted_slot_bit = JobFlags::accepted_slot_bit_for_job_code(code)?;

    crate::ffxiv::JOBS.iter().find_map(|(&job_id, job)| {
        (job.code() == code
            && JobFlags::accepted_slot_bit_for_job_id(job_id) == Some(accepted_slot_bit))
            .then_some(job_id)
    })
}

fn canonical_job_ids_for_codes(codes: &str) -> Vec<u32> {
    let mut seen = HashSet::new();

    codes
        .split_whitespace()
        .filter_map(canonical_job_id_for_code)
        .filter(|job_id| seen.insert(*job_id))
        .collect()
}

// Slot job-code strings are public v1 contract data, so keep canonical ID coverage current
// through Beastmaster instead of treating PCT as the last renderable job token.

fn slot_matches_any_accepted_slot_bit(
    slot: &crate::listing::PartyFinderSlot,
    accepted_slot_bits: &[u64],
) -> bool {
    accepted_slot_bits
        .iter()
        .any(|accepted_slot_bit| u64::from(slot.accepting.bits()) & accepted_slot_bit != 0)
}

fn build_job_match_conditions(accepted_slot_bits: &[u64]) -> Vec<mongodb::bson::Document> {
    accepted_slot_bits
        .iter()
        .map(|&accepted_slot_bit| {
            let accepted_slot_bit =
                i64::try_from(accepted_slot_bit).expect("accepted slot bit must fit Mongo bit query");

            doc! {
                "listing.slots": {
                    "$elemMatch": {
                        "accepting": {
                            "$bitsAllSet": accepted_slot_bit
                        }
                    }
                }
            }
        })
        .collect()
}

fn slot_info_from_listing_slot(
    slot_result: &std::result::Result<ffxiv_types_cn::jobs::ClassJob, (String, String)>,
) -> SlotInfo {
    match slot_result {
        Ok(job) => SlotInfo {
            filled: true,
            role: job.role().map(|r| r.to_string()),
            role_id: match job.role() {
                Some(ffxiv_types_cn::Role::Tank) => 1,
                Some(ffxiv_types_cn::Role::Healer) => 2,
                Some(ffxiv_types_cn::Role::Dps) => 3,
                None => 0,
            },
            job: Some(job.code().to_string()),
            job_id: canonical_job_ids_for_codes(job.code()),
        },
        Err((role_class, job_code)) => SlotInfo {
            filled: false,
            role: if role_class.contains("tank") {
                Some("Tank".to_string())
            } else if role_class.contains("healer") {
                Some("Healer".to_string())
            } else if role_class.contains("dps") {
                Some("DPS".to_string())
            } else {
                None
            },
            role_id: if role_class.contains("tank") {
                1
            } else if role_class.contains("healer") {
                2
            } else if role_class.contains("dps") {
                3
            } else {
                0
            },
            job: if job_code.is_empty() {
                None
            } else {
                Some(job_code.clone())
            },
            job_id: canonical_job_ids_for_codes(job_code),
        },
    }
}

/// 获取招募列表的API
/// 
/// 支持以下查询参数:
/// - page: 页码，默认为1
/// - per_page: 每页数量，默认为20，最大为100
/// - category: 分类过滤
/// - world: 世界过滤
/// - search: 搜索关键词，会匹配名称和描述
/// - datacenter: 数据中心过滤，支持多个数据中心，用逗号分隔，如"猫小胖,豆豆柴"
/// - jobs: 职业过滤，支持多个职业ID，用逗号分隔，如"1,2,43"
/// - duty: 副本过滤，支持多个副本ID，用逗号分隔，如"1,2,8"
///
/// 职业ID对应关系:
/// 请参考jobs.rs中的hashmap，Beastmaster 对外使用 `BST` / `43`
/// 示例:
/// GET /api/listings?page=1&per_page=20&category=None&world=拉诺西亚&jobs=8,10,21
/// GET /api/listings?page=1&per_page=20&datacenter=猫小胖&category=HighEndDuty&jobs=10,21
/// GET /api/listings?page=1&per_page=20&duty=1,2,3&jobs=8,10
/// GET /api/listings?page=1&per_page=20&jobs=43
pub fn listings_api(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(
        state: Arc<State>, 
        page: Option<usize>, 
        per_page: Option<usize>,
        category: Option<String>,
        world: Option<String>,
        search: Option<String>,
        datacenter: Option<String>,
        jobs: Option<String>,
        duty: Option<String>,
    ) -> std::result::Result<impl Reply, Infallible> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(20).min(100); // 限制每页最大数量为100
        
        // 转换category为DutyCategory
        let category = category.and_then(|cat| {
            match cat.as_str() {
                "None" => Some(DutyCategory::None),
                "DutyRoulette" => Some(DutyCategory::DutyRoulette),
                "Dungeons" => Some(DutyCategory::Dungeon),
                "Guildhests" => Some(DutyCategory::Guildhest),
                "Trials" => Some(DutyCategory::Trial),
                "Raids" => Some(DutyCategory::Raid),
                "HighEndDuty" => Some(DutyCategory::HighEndDuty),
                "Pvp" => Some(DutyCategory::PvP),
                "GoldSaucer" => Some(DutyCategory::GoldSaucer),
                "Fates" => Some(DutyCategory::Fate),
                "TreasureHunt" => Some(DutyCategory::TreasureHunt),
                "TheHunt" => Some(DutyCategory::TheHunt),
                "GatheringForays" => Some(DutyCategory::GatheringForays),
                "DeepDungeons" => Some(DutyCategory::DeepDungeon),
                "FieldOperations" => Some(DutyCategory::FieldOperation),
                "V&C Dungeon Finder" => Some(DutyCategory::VariantAndCriterionDungeon),
                _ => None,
            }
        });

        // 转换world为World，如果提供了world但找不到匹配的，直接返回空结果
        let world = if let Some(w) = world {
            match crate::ffxiv::WORLDS.values().find(|world| world.name() == w) {
                Some(found_world) => Some(found_world),
                None => {
                    // 如果提供了world但找不到匹配的，返回空结果
                    let pagination = Pagination {
                        total: 0,
                        page,
                        per_page,
                        total_pages: 0,
                    };
                    
                    let response = ApiResponse {
                        data: Vec::<ApiListing>::new(),
                        pagination,
                    };
                    
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&response),
                        StatusCode::OK,
                    ));
                }
            }
        } else {
            None
        };

        // 验证数据中心是否存在
        let mut datacenter_list = Vec::new();
        if let Some(dc_str) = datacenter.as_deref() {
            let all_dcs: std::collections::HashSet<String> = crate::ffxiv::WORLDS.values()
                .map(|w| w.data_center().name().to_string())
                .collect();
            for dc_name in dc_str.split(',').map(|s| s.trim()) {
                if all_dcs.contains(dc_name) {
                    datacenter_list.push(dc_name.to_string());
                }
            }
        }

        // 如果提供了datacenter参数但没有有效的，返回空结果
        if datacenter.is_some() && datacenter_list.is_empty() {
            let pagination = Pagination {
                total: 0,
                page,
                per_page,
                total_pages: 0,
            };
            
            let response = ApiResponse {
                data: Vec::<ApiListing>::new(),
                pagination,
            };
            
            return Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ));
        }

        // 验证职业ID是否合法
        let mut job_list = Vec::new();
        // 处理逗号分隔的格式
        if let Some(jobs_str) = jobs.as_deref() {
            for job_id in jobs_str.split(',').filter_map(|s| s.trim().parse::<u32>().ok()) {
                if JobFlags::accepted_slot_bit_for_job_id(job_id).is_some() {
                    job_list.push(job_id);
                }
            }
        }

        // 如果提供了职业参数但没有有效的职业ID，返回空结果
        if jobs.is_some() && job_list.is_empty() {
            let pagination = Pagination {
                total: 0,
                page,
                per_page,
                total_pages: 0,
            };
            
            let response = ApiResponse {
                data: Vec::<ApiListing>::new(),
                pagination,
            };
            
            return Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ));
        }
        
        // 处理副本ID列表
        let mut duty_list = Vec::new();
        // 处理逗号分隔的格式
        if let Some(duty_str) = duty.as_deref() {
            for duty_id in duty_str.split(',').filter_map(|s| s.trim().parse::<u16>().ok()) {
                duty_list.push(duty_id);
            }
        }

        // 如果提供了副本参数但没有有效的副本ID，返回空结果
        if duty.is_some() && duty_list.is_empty() {
            let pagination = Pagination {
                total: 0,
                page,
                per_page,
                total_pages: 0,
            };
            
            let response = ApiResponse {
                data: Vec::<ApiListing>::new(),
                pagination,
            };
            
            return Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ));
        }
        
        // 对job_list和duty_list进行排序，确保不同顺序的相同参数可以命中相同的缓存
        job_list.sort_unstable();
        job_list.dedup(); // 移除重复项
        duty_list.sort_unstable();
        duty_list.dedup(); // 移除重复项
        datacenter_list.sort_unstable();
        datacenter_list.dedup();
        let accepted_slot_bits = accepted_slot_bits_for_job_ids(&job_list);
        
        // 构建缓存键 - 使用jobs参数和duty参数
        let cache_key = format!(
            "listings_p{}_pp{}_c{}_w{}_s{}_dc{}_js{}_du{}", 
            page, 
            per_page, 
            category.map(|c| c.pf_category().as_str()).unwrap_or(""),
            world.as_deref().map(|w| w.name()).unwrap_or(""), 
            search.as_deref().unwrap_or(""), 
            datacenter_list.join("_"),
            job_list.iter().map(|j| j.to_string()).collect::<Vec<String>>().join("_"),
            duty_list.iter().map(|d| d.to_string()).collect::<Vec<String>>().join("_")
        );
        
        // 尝试从缓存获取
        if let Some(cached) = state.get_listings_cache(&cache_key).await {
            return Ok(warp::reply::with_status(
                warp::reply::json(&cached),
                StatusCode::OK,
            ));
        }
        
        let lang = Language::ChineseSimplified;

        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        
        let mut pipeline = vec![
            // 1. 首先进行基础过滤，尽早减少数据量
            doc! {
                "$match": {
                    "updated_at": { "$gte": two_hours_ago },
                    // 过滤私有PF
                    "listing.search_area": { "$bitsAllClear": 2 },
                }
            },
        ];

        // 2. 添加分类过滤条件 - 提前过滤
        if let Some(cat) = &category {
            pipeline.push(doc! {
                "$match": {
                    "listing.category": *cat as u32,
                }
            });
        }

        // 3. 添加世界/数据中心过滤条件 - 提前过滤
        if let Some(w) = &world {
            let world_id = crate::ffxiv::WORLDS.iter().find(|(_, world)| world.name() == w.name()).map(|(id, _)| *id as u32);
            
            if let Some(id) = world_id {
                pipeline.push(doc! {
                    "$match": {
                        "listing.created_world": id
                    }
                });
            }
        } else if !datacenter_list.is_empty() {
            let world_ids: Vec<u32> = crate::ffxiv::WORLDS.iter()
                .filter(|(_, world)| datacenter_list.contains(&world.data_center().name().to_string()))
                .map(|(id, _)| *id)
                .collect();

            if !world_ids.is_empty() {
                let world_ids_i32: Vec<i32> = world_ids.iter().map(|&id| id as i32).collect();
                pipeline.push(doc! {
                    "$match": {
                        "listing.created_world": { "$in": world_ids_i32 }
                    }
                });
            }
        }

        // 3.5 添加副本ID过滤条件 - 提前过滤
        if !duty_list.is_empty() {
            let duty_ids_i32: Vec<i32> = duty_list.iter().map(|&id| id as i32).collect();
            pipeline.push(doc! {
                "$match": {
                    "listing.duty": { "$in": duty_ids_i32 }
                }
            });
        }

        // 4. 添加职业过滤条件 - 提前过滤
        if !accepted_slot_bits.is_empty() {
            let job_conditions = build_job_match_conditions(&accepted_slot_bits);

            if !job_conditions.is_empty() {
                pipeline.push(doc! {
                    "$match": {
                        "$or": job_conditions
                    }
                });
            }
        }

        // 5. 计算时间相关字段
        pipeline.push(doc! {
            "$set": {
                "time_left": {
                    "$divide": [
                        {
                            "$subtract": [
                                { "$multiply": ["$listing.seconds_remaining", 1000] },
                                { "$subtract": ["$$NOW", "$updated_at"] },
                            ]
                        },
                        1000,
                    ]
                },
                "minutes_since_update": {
                    "$divide": [
                        { "$subtract": ["$$NOW", "$updated_at"] },
                        60000
                    ]
                }
            }
        });

        // 6. 过滤过期招募（基于剩余时间或超过5分钟未更新）
        pipeline.push(doc! {
            "$match": {
                "$and": [
                    { "time_left": { "$gte": 0 } },
                    { "minutes_since_update": { "$lt": 5.0 } }
                ]
            }
        });

        // 7. 按content_id_lower分组前先排序，确保获取最新的招募
        pipeline.push(doc! {
            "$sort": {
                "updated_at": -1
            }
        });

        // 8. 分组获取每个玩家最新的招募
        pipeline.push(doc! {
            "$group": {
                "_id": "$listing.content_id_lower",
                "doc": { "$first": "$$ROOT" }
            }
        });

        // 9. 恢复文档结构
        pipeline.push(doc! {
            "$replaceRoot": { "newRoot": "$doc" }
        });

        // 10. 最后添加分页相关的排序和时间分组
        pipeline.push(doc! {
            "$set": {
                "updated_minute": {
                    "$dateTrunc": {
                        "date": "$updated_at",
                        "unit": "minute",
                        "binSize": 5,
                    },
                }
            }
        });

        // 11. 最终排序
        pipeline.push(doc! {
            "$sort": {
                "updated_minute": -1,
                "listing.pf_category": -1,
                "time_left": 1,
            }
        });
        
        // 如果有搜索条件，我们需要在Rust端处理
        // 但如果没有搜索条件，可以直接在MongoDB中分页
        if search.is_none() {
            // 添加计数阶段以获取总数
            pipeline.push(doc! {
                "$facet": {
                    "metadata": [{ "$count": "total" }],
                    "data": [
                        { "$skip": ((page - 1) * per_page) as i64 },
                        { "$limit": per_page as i64 }
                    ]
                }
            });
        }
        
        // 执行主查询
        let res = state
            .collection()
            .aggregate(pipeline, None)
            .await;
            
        let reply = match res {
            Ok(mut cursor) => {
                // 如果没有搜索条件，直接使用MongoDB的分页结果
                if search.is_none() {
                    if let Ok(Some(result)) = cursor.try_next().await {
                        // 将结果转换为Document
                        let facet_result = match mongodb::bson::from_document::<mongodb::bson::Document>(result) {
                            Ok(doc) => doc,
                            Err(_) => {
                                // 返回空结果
                                let pagination = Pagination {
                                    total: 0,
                                    page,
                                    per_page,
                                    total_pages: 0,
                                };
                                
                                let response = ApiResponse {
                                    data: Vec::<ApiListing>::new(),
                                    pagination,
                                };
                                
                                return Ok(warp::reply::with_status(
                                    warp::reply::json(&response),
                                    StatusCode::OK,
                                ));
                            }
                        };
                        
                        // 提取元数据
                        let total = match facet_result.get_array("metadata") {
                            Ok(metadata) if !metadata.is_empty() => {
                                match metadata[0].as_document() {
                                    Some(doc) => match doc.get_i32("total") {
                                        Ok(count) => count as usize,
                                        Err(_) => 0
                                    },
                                    None => 0
                                }
                            },
                            _ => 0
                        };
                        
                        let total_pages = (total + per_page - 1) / per_page;
                        
                        // 提取数据
                        let mut api_listings = Vec::new();
                        if let Ok(data) = facet_result.get_array("data") {
                            for item in data {
                                if let Some(doc) = item.as_document() {
                                    if let Ok(container) = mongodb::bson::from_document::<QueriedListing>(doc.clone()) {
                                        let listing = &container.listing;
                                        api_listings.push(ApiListing {
                                            id: listing.id,
                                            name: listing.name.full_text(&lang).to_string(),
                                            description: listing.description.full_text(&lang).to_string(),
                                            created_world: listing.created_world_string().to_string(),
                                            created_world_id: u32::from(listing.created_world),
                                            home_world: listing.home_world_string().to_string(),
                                            home_world_id: u32::from(listing.home_world),
                                            category: listing.pf_category().as_str().to_string(),
                                            category_id: listing.category.as_u32(),
                                            duty: listing.duty_name(&lang).to_string(),
                                            min_item_level: listing.min_item_level,
                                            slots_filled: listing.slots_filled(),
                                            slots_available: listing.slots_available,
                                            time_left: container.time_left,
                                            updated_at: container.updated_at.to_rfc3339(),
                                            is_cross_world: listing.is_cross_world(),
                                            datacenter: listing.data_centre_name().map(|dc| dc.to_string()),
                                        });
                                    }
                                }
                            }
                        }
                        
                        let pagination = Pagination {
                            total,
                            page,
                            per_page,
                            total_pages,
                        };
                        
                        let response = ApiResponse {
                            data: api_listings,
                            pagination,
                        };
                        
                        // 缓存结果 - 设置30秒的TTL
                        state.set_listings_cache(cache_key, response.clone(), 30).await;

                        return Ok(warp::reply::with_status(
                            warp::reply::json(&response),
                            StatusCode::OK,
                        ));
                    }
                    
                    // 如果没有结果，返回空数组
                    let pagination = Pagination {
                        total: 0,
                        page,
                        per_page,
                        total_pages: 0,
                    };
                    
                    let response = ApiResponse {
                        data: Vec::<ApiListing>::new(),
                        pagination,
                    };
                    
                    return Ok(warp::reply::with_status(
                        warp::reply::json(&response),
                        StatusCode::OK,
                    ));
                }
                
                // 如果有搜索条件，需要在Rust端处理
                let mut containers = Vec::new();

                while let Ok(Some(container)) = cursor.try_next().await {
                    let res: anyhow::Result<QueriedListing> = try {
                        let result: QueriedListing = mongodb::bson::from_document(container)?;
                        result
                    };
                    if let Ok(listing) = res {
                        containers.push(listing);
                    }
                }

                // 在Rust端进行搜索过滤
                let mut filtered_containers = Vec::new();
                
                // 应用搜索过滤条件
                if let Some(s) = &search {
                    let search_lower = s.to_lowercase();
                    for container in containers {
                        let listing = &container.listing;
                        let name = listing.name.full_text(&lang).to_string().to_lowercase();
                        let description = listing.description.full_text(&lang).to_string().to_lowercase();
                        if name.contains(&search_lower) || description.contains(&search_lower) {
                            // 如果有职业过滤，再次检查（以防MongoDB查询不完整）
                            if !job_list.is_empty() {
                                let has_job = listing
                                    .slots
                                    .iter()
                                    .any(|slot| slot_matches_any_accepted_slot_bit(slot, &accepted_slot_bits));

                                if has_job {
                                    filtered_containers.push(container);
                                }
                            } else {
                                filtered_containers.push(container);
                            }
                        }
                    }
                } else {
                    // 如果没有搜索条件，但有职业过滤
                    if !job_list.is_empty() {
                        for container in containers {
                            let listing = &container.listing;
                            let has_job = listing
                                .slots
                                .iter()
                                .any(|slot| slot_matches_any_accepted_slot_bit(slot, &accepted_slot_bits));
                            
                            if has_job {
                                filtered_containers.push(container);
                            }
                        }
                    } else {
                        filtered_containers = containers;
                    }
                }
                
                // 计算总数和分页
                let total = filtered_containers.len();
                let total_pages = (total + per_page - 1) / per_page;
                
                // 手动分页
                let start = (page - 1) * per_page;
                
                // 检查页码是否有效
                let api_listings = if page > total_pages && total > 0 {
                    // 如果请求的页码超出范围且有数据，返回空数组
                    Vec::new()
                } else {
                    // 正常分页逻辑
                    let start = if start >= total { 0 } else { start };
                    let items_left = if total > start { total - start } else { 0 };
                    let end = start + per_page.min(items_left);
                    
                    // 转换为API响应格式
                    filtered_containers
                        .iter()
                        .skip(start)
                        .take(end - start)
                        .map(|container| {
                            let listing = &container.listing;
                            ApiListing {
                                id: listing.id,
                                name: listing.name.full_text(&lang).to_string(),
                                description: listing.description.full_text(&lang).to_string(),
                                created_world: listing.created_world_string().to_string(),
                                created_world_id: u32::from(listing.created_world),
                                home_world: listing.home_world_string().to_string(),
                                home_world_id: u32::from(listing.home_world),
                                category: listing.pf_category().as_str().to_string(),
                                category_id: listing.category.as_u32(),
                                duty: listing.duty_name(&lang).to_string(),
                                min_item_level: listing.min_item_level,
                                slots_filled: listing.slots_filled(),
                                slots_available: listing.slots_available,
                                time_left: container.time_left,
                                updated_at: container.updated_at.to_rfc3339(),
                                is_cross_world: listing.is_cross_world(),
                                datacenter: listing.data_centre_name().map(|dc| dc.to_string()),
                            }
                        }).collect()
                };

                // 使用请求的原始页码
                let pagination = Pagination {
                    total,
                    page,
                    per_page,
                    total_pages,
                };

                let response = ApiResponse {
                    data: api_listings,
                    pagination,
                };

                // 缓存结果 - 设置30秒的TTL
                state.set_listings_cache(cache_key, response.clone(), 30).await;

                return Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    StatusCode::OK,
                ));
            },
            Err(e) => {
                eprintln!("{:#?}", e);
                let pagination = Pagination {
                    total: 0,
                    page,
                    per_page,
                    total_pages: 0,
                };
                
                let response = ApiResponse {
                    data: Vec::<ApiListing>::new(),
                    pagination,
                };
                
                warp::reply::with_status(
                    warp::reply::json(&response),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        };
        
        Ok(reply)
    }

    // 使用原始的路由实现方式，避免复杂的类型问题
    let state_clone = state.clone();
    warp::path("api")
        .and(warp::path("listings"))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::any().map(move || state_clone.clone()))
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .then(move |state: Arc<State>, params: std::collections::HashMap<String, String>| {
            let page = params.get("page").and_then(|v| v.parse().ok());
            let per_page = params.get("per_page").and_then(|v| v.parse().ok());
            let category = params.get("category").cloned();
            let world = params.get("world").cloned();
            let search = params.get("search").cloned();
            let datacenter = params.get("datacenter").cloned();
            let jobs = params.get("jobs").cloned();
            let duty = params.get("duty").cloned();
            
            let state_clone = state.clone();
            async move {
                logic(state_clone, page, per_page, category, world, search, datacenter, jobs, duty).await
            }
        })
        .boxed()
}

// 添加获取单个招募详细信息的API
pub fn listing_detail_api(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(
        state: Arc<State>,
        id: u64,
    ) -> std::result::Result<impl Reply, Infallible> {
        // 尝试从缓存获取
        if let Some(cached) = state.get_detail_cache(id).await {
            return Ok(warp::reply::with_status(
                warp::reply::json(&cached),
                StatusCode::OK,
            ));
        }
        
        let lang = Language::ChineseSimplified;
        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        
        // 简化查询 - 合并多个$match阶段
        let listing_id = match to_bson(&id) {
            Ok(listing_id) => listing_id,
            Err(error) => {
                eprintln!("序列化招募ID错误: {:#?}", error);
                return Ok(warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({
                        "error": "数据库查询错误"
                    })),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        };
        let pipeline = vec![
            doc! {
                "$match": {
                    "updated_at": { "$gte": two_hours_ago },
                    "listing.id": listing_id,
                    // 过滤私有PF
                    "listing.search_area": { "$bitsAllClear": 2 },
                }
            },
            doc! {
                "$set": {
                    "time_left": {
                        "$divide": [
                            {
                                "$subtract": [
                                    { "$multiply": ["$listing.seconds_remaining", 1000] },
                                    { "$subtract": ["$$NOW", "$updated_at"] },
                                ]
                            },
                            1000,
                        ]
                    },
                    "minutes_since_update": {
                        "$divide": [
                            { "$subtract": ["$$NOW", "$updated_at"] },
                            60000
                        ]
                    },
                    "updated_minute": {
                        "$dateTrunc": {
                            "date": "$updated_at",
                            "unit": "minute",
                            "binSize": 5,
                        },
                    },
                }
            },
            doc! {
                "$match": {
                    "$and": [
                        { "time_left": { "$gte": 0 } },
                        { "minutes_since_update": { "$lt": 5.0 } }
                    ]
                }
            },
            doc! {
                "$sort": {
                    "updated_at": -1,
                }
            },
            doc! {
                "$limit": 1
            },
        ];
        
        // 执行查询
        let res = state
            .collection()
            .aggregate(pipeline, None)
            .await;
            
        Ok(match res {
            Ok(mut cursor) => {
                if let Ok(Some(container)) = cursor.try_next().await {
                    let res: anyhow::Result<QueriedListing> = try {
                        let result: QueriedListing = mongodb::bson::from_document(container)?;
                        result
                    };
                    
                    if let Ok(container) = res {
                        let listing = &container.listing;
                        
                        // 构建槽位信息 - 使用迭代器而不是collect以提高性能
                        let mut slots = Vec::with_capacity(listing.slots().len());
                        for (_i, slot_result) in listing.slots().iter().enumerate() {
                            slots.push(slot_info_from_listing_slot(slot_result));
                        }
                        
                        // 构建详细信息
                        let detailed = DetailedApiListing {
                            id: listing.id,
                            name: listing.name.full_text(&lang).to_string(),
                            description: listing.description.full_text(&lang).to_string(),
                            created_world: listing.created_world_string().to_string(),
                            home_world: listing.home_world_string().to_string(),
                            category: listing.pf_category().as_str().to_string(),
                            duty: listing.duty_name(&lang).to_string(),
                            min_item_level: listing.min_item_level,
                            slots_filled: listing.slots_filled(),
                            slots_available: listing.slots_available,
                            time_left: container.time_left,
                            updated_at: container.updated_at.to_rfc3339(),
                            is_cross_world: listing.is_cross_world(),
                            // 添加更多详细信息
                            beginners_welcome: listing.beginners_welcome,
                            duty_type: format!("{:?}", listing.duty_type),
                            objective: format!("{:?}", listing.objective),
                            conditions: format!("{:?}", listing.conditions),
                            loot_rules: format!("{:?}", listing.loot_rules),
                            slots,
                            datacenter: listing.data_centre_name().map(|dc| dc.to_string()),
                        };
                        
                        // 缓存结果 - 设置60秒的TTL
                        state.set_detail_cache(id, detailed.clone(), 60).await;
                        
                        warp::reply::with_status(
                            warp::reply::json(&detailed),
                            StatusCode::OK,
                        )
                    } else {
                        // 添加错误日志
                        if let Err(e) = res {
                            eprintln!("解析招募信息错误: {:#?}", e);
                        }
                        
                        warp::reply::with_status(
                            warp::reply::json(&serde_json::json!({
                                "error": "无法解析招募信息"
                            })),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )
                    }
                } else {
                    warp::reply::with_status(
                        warp::reply::json(&serde_json::json!({
                            "error": "未找到招募信息"
                        })),
                        StatusCode::NOT_FOUND,
                    )
                }
            },
            Err(e) => {
                eprintln!("获取招募详情错误: {:#?}", e);
                warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({
                        "error": "数据库查询错误"
                    })),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }
        })
    }

    let route = warp::path("api")
        .and(warp::path("listing"))
        .and(warp::path::param::<u64>())
        .and(warp::path::end())
        .and_then(move |id: u64| logic(Arc::clone(&state), id));

    warp::get().and(route).boxed()
}

#[cfg(test)]
pub(crate) async fn listing_detail_api_with_cache(
    state: Arc<State>,
    id: u64,
    detail: DetailedApiListing,
) -> Arc<State> {
    state.set_detail_cache(id, detail, 60).await;
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use ffxiv_types_cn::jobs::{ClassJob, Job};
    use mongodb::bson::doc;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn v1_detail_returns_exact_wide_numeric_id_via_cache() {
        let state = crate::web::state_for_router_tests().await;
        
        let wide_id: u64 = 4_294_967_296;
        let detail = DetailedApiListing {
            id: wide_id,
            name: "TestPlayer".into(),
            description: "Test description".into(),
            created_world: "拉诺西亚".into(),
            home_world: "拉诺西亚".into(),
            category: "HighEndDuty".into(),
            duty: "亚历山大零式".into(),
            min_item_level: 710,
            slots_filled: 4,
            slots_available: 8,
            time_left: 1200.0,
            updated_at: "2026-05-02T12:00:00Z".into(),
            is_cross_world: true,
            beginners_welcome: false,
            duty_type: "Normal".into(),
            objective: "DutyCompletion".into(),
            conditions: "DutyComplete".into(),
            loot_rules: "Lootmaster".into(),
            slots: vec![],
            datacenter: Some("猫小胖".into()),
        };
        
        let state = listing_detail_api_with_cache(state, wide_id, detail).await;
        
        let route = listing_detail_api(state);
        
        let response = warp::test::request()
            .method("GET")
            .path(&format!("/api/listing/{}", wide_id))
            .reply(&route)
            .await;
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(body["id"], wide_id, "v1 should return exact widened numeric ID");
    }

    #[test]
    fn beastmaster_jobs_query_uses_canonical_slot_bit() {
        assert_eq!(accepted_slot_bits_for_job_ids(&[43]), vec![1u64 << 32]);

        let job_conditions = build_job_match_conditions(&accepted_slot_bits_for_job_ids(&[43]));

        assert_eq!(
            job_conditions,
            vec![doc! {
                "listing.slots": {
                    "$elemMatch": {
                        "accepting": {
                            "$bitsAllSet": 4_294_967_296i64,
                        }
                    }
                }
            }]
        );
    }

    #[test]
    fn detail_slot_serializes_beastmaster_as_bst_and_43() {
        let slot_info = slot_info_from_listing_slot(&Ok(ClassJob::Job(Job::Beastmaster)));

        assert!(slot_info.filled);
        assert_eq!(slot_info.job.as_deref(), Some("BST"));
        assert_eq!(slot_info.job_id, vec![43]);
    }
}
