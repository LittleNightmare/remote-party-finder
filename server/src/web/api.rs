use std::{
    convert::Infallible,
    sync::Arc,
};

use chrono::Utc;
use mongodb::bson::doc;
use serde::Serialize;
use tokio_stream::StreamExt;
use warp::{
    Filter,
    filters::BoxedFilter,
    http::StatusCode,
    Reply,
};

use crate::{
    ffxiv::Language,
    listing_container::QueriedListing,
    web::State,
    sestring_ext::SeStringExt,
};

#[derive(Serialize)]
struct ApiResponse<T> {
    data: T,
    pagination: Pagination,
}

#[derive(Serialize)]
struct Pagination {
    total: usize,
    page: usize,
    per_page: usize,
    total_pages: usize,
}

#[derive(Serialize)]
struct ApiListing {
    id: u32,
    name: String,
    description: String,
    created_world: String,
    home_world: String,
    category: String,
    duty: String,
    min_item_level: u16,
    slots_filled: usize,
    slots_available: u8,
    time_left: f64,
    updated_at: String,
    is_cross_world: bool,
    datacenter: Option<String>,
}

pub fn listings_api(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(
        state: Arc<State>, 
        page: Option<usize>, 
        per_page: Option<usize>,
        category: Option<String>,
        world: Option<String>,
        search: Option<String>,
        datacenter: Option<String>,
    ) -> std::result::Result<impl Reply, Infallible> {
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(20).min(100); // 限制每页最大数量为100
        
        // 打印接收到的参数
        println!("API请求参数: page={:?}, per_page={:?}, category={:?}, world={:?}, search={:?}, datacenter={:?}", 
                 page, per_page, category, world, search, datacenter);
        
        let lang = Language::ChineseSimplified;

        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        
        // 构建基本查询 - 不包含分页，获取所有符合条件的数据
        let mut pipeline = vec![
            doc! {
                "$match": {
                    "updated_at": { "$gte": two_hours_ago },
                }
            },
            doc! {
                "$match": {
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
                    "time_left": { "$gte": 0 },
                }
            },
        ];
        
        // 添加排序
        pipeline.push(doc! {
            "$sort": {
                "updated_minute": -1,
                "listing.pf_category": -1,
                "time_left": 1,
            }
        });
        
        // 执行主查询 - 获取所有符合条件的数据
        let res = state
            .collection()
            .aggregate(pipeline, None)
            .await;
            
        println!("MongoDB查询结果: {:?}", res.is_ok());
        
        Ok(match res {
            Ok(mut cursor) => {
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

                // 在Rust端进行过滤
                let mut filtered_containers = Vec::new();
                
                // 应用所有过滤条件
                for container in containers {
                    let mut include = true;
                    let listing = &container.listing;
                    
                    // 分类过滤
                    if let Some(cat) = &category {
                        if listing.pf_category().as_str() != *cat {
                            include = false;
                        }
                    }
                    
                    // 世界过滤
                    if include && world.is_some() {
                        let w = world.as_ref().unwrap();
                        if listing.created_world_string().to_string() != *w && 
                           listing.home_world_string().to_string() != *w {
                            include = false;
                        }
                    }
                    
                    // 数据中心过滤
                    if include && datacenter.is_some() {
                        let dc = datacenter.as_ref().unwrap();
                        let listing_dc = listing.data_centre_name();
                        
                        if let Some(listing_dc_name) = listing_dc {
                            if listing_dc_name != dc {
                                include = false;
                            }
                        } else {
                            include = false;
                        }
                    }
                    
                    // 搜索过滤
                    if include && search.is_some() {
                        let s = search.as_ref().unwrap();
                        let search_lower = s.to_lowercase();
                        let name = listing.name.full_text(&lang).to_string().to_lowercase();
                        let description = listing.description.full_text(&lang).to_string().to_lowercase();
                        if !name.contains(&search_lower) && !description.contains(&search_lower) {
                            include = false;
                        }
                    }
                    
                    if include {
                        filtered_containers.push(container);
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
                    println!("请求的页码 {} 超出范围，总页数为 {}", page, total_pages);
                    Vec::new()
                } else {
                    // 正常分页逻辑
                    let start = if start >= total { 0 } else { start };
                    let items_left = if total > start { total - start } else { 0 };
                    let end = start + per_page.min(items_left);
                    
                    println!("分页信息: total={}, page={}, per_page={}, start={}, end={}, total_pages={}", 
                             total, page, per_page, start, end, total_pages);
                    
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
                                home_world: listing.home_world_string().to_string(),
                                category: listing.pf_category().as_str().to_string(),
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

                warp::reply::with_status(
                    warp::reply::json(&response),
                    StatusCode::OK,
                )
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
        })
    }

    let route = warp::path("api")
        .and(warp::path("listings"))
        .and(warp::path::end())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|params: std::collections::HashMap<String, String>| {
            let page = params.get("page").and_then(|p| p.parse::<usize>().ok());
            let per_page = params.get("per_page").and_then(|p| p.parse::<usize>().ok());
            let category = params.get("category").cloned();
            let world = params.get("world").cloned();
            let search = params.get("search").cloned();
            let datacenter = params.get("datacenter").cloned();
            (page, per_page, category, world, search, datacenter)
        })
        .and_then(move |(page, per_page, category, world, search, datacenter)| {
            logic(Arc::clone(&state), page, per_page, category, world, search, datacenter)
        });

    warp::get().and(route).boxed()
}

// 添加获取单个招募详细信息的API
pub fn listing_detail_api(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(
        state: Arc<State>,
        id: u32,
    ) -> std::result::Result<impl Reply, Infallible> {
        println!("API请求招募详情: id={}", id);
        
        let lang = Language::ChineseSimplified;
        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        
        // 构建查询
        let pipeline = vec![
            doc! {
                "$match": {
                    "updated_at": { "$gte": two_hours_ago },
                    "listing.id": id,
                }
            },
            doc! {
                "$match": {
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
                    "time_left": { "$gte": 0 },
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
            
        println!("MongoDB查询结果: {:?}", res.is_ok());
        
        #[derive(Serialize)]
        struct DetailedApiListing {
            id: u32,
            name: String,
            description: String,
            created_world: String,
            home_world: String,
            category: String,
            duty: String,
            min_item_level: u16,
            slots_filled: usize,
            slots_available: u8,
            time_left: f64,
            updated_at: String,
            is_cross_world: bool,
            // 添加更多详细信息
            beginners_welcome: bool,
            duty_type: String,
            objective: String,
            conditions: String,
            loot_rules: String,
            slots: Vec<SlotInfo>,
            datacenter: Option<String>,
        }
        
        #[derive(Serialize)]
        struct SlotInfo {
            filled: bool,
            role: Option<String>,
            job: Option<String>,
        }
            
        Ok(match res {
            Ok(mut cursor) => {
                if let Ok(Some(container)) = cursor.try_next().await {
                    println!("成功获取到招募数据");
                    let res: anyhow::Result<QueriedListing> = try {
                        let result: QueriedListing = mongodb::bson::from_document(container)?;
                        result
                    };
                    
                    if let Ok(container) = res {
                        let listing = &container.listing;
                        
                        // 构建槽位信息
                        let slots = listing.slots().iter().enumerate().map(|(_i, slot_result)| {
                            match slot_result {
                                Ok(job) => SlotInfo {
                                    filled: true,
                                    role: job.role().map(|r| r.to_string()),
                                    job: Some(job.code().to_string()),
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
                                    job: if job_code.is_empty() { None } else { Some(job_code.clone()) },
                                },
                            }
                        }).collect();
                        
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
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and_then(move |id: u32| logic(Arc::clone(&state), id));

    warp::get().and(route).boxed()
} 