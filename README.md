# Remote Party Finder
A fork version of Remote Party Finder from [Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)

this repo this prearing for CN server

Fork了[Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)的Remote Party Finder

这个仓库是准备为了国服构建的，但我本人对server端的内容真的不熟。希望有人能帮帮我orz

## 关于显示FFXIV的特殊符号

参考这个[issue](https://github.com/LittleNightmare/remote-party-finder/issues/3)，下载对应的字体放在`server/assets`里

## API
已经提供了 API，使用方式请见仓库内文档：

- [Legacy / v1 API](docs/api-v1.md)
- [API v2 phase 1](docs/api-v2.md)

### API v2 phase 1

This repo now ships a parallel read API under `/api/v2`.

- v1 stays available. v2 is additive and runs in parallel during migration.
- Phase 1 exposes only `GET /api/v2/listings` and `GET /api/v2/listings/{id}`.
- Listing resources are IDs-only for lookup-backed fields such as worlds, categories, duties, jobs, objectives, conditions, loot rules, and slot roles.
- Phase 1 has no `/api/v2/lookups/*` routes. Clients must resolve labels outside this API.
- `/api/v2/listings/{id}` is an active-detail lookup alias for the current visible PF listing id. It is not a durable historical identity.

See [`docs/api-v2.md`](docs/api-v2.md) for the phase-1 contract, examples, and migration notes.

## 前端
可以查看利用 API 的前端项目：[remote-party-finder-frontend](https://github.com/Cindy-Master/remote-party-finder-frontend)。

[点击这里访问xivpf.littlengihtmare.top](https://xivpf.littlenightmare.top)

## To Anna
If you don't wanna see it on Github, please tell me through issue.

## 已知问题

Cloudflare API Shield 的 Schema Validation 在 Free 套餐下存在未文档化的 ~1KB 请求体检查限制，导致较大的 `POST /contribute/multiple` 请求被 403 拒绝。目前已临时关闭 Schema Validation，并在服务端 (`web.rs`) 添加了 `created_world` 范围校验 (1000-1999) 作为替代。详见 [#13](https://github.com/LittleNightmare/remote-party-finder/issues/13)。

### TODO: 未来 PF 新增 category 位时的兼容

7.5 新增的斗兽奇弈（ContentType 表第 40 行，官方占位符名 `●XBM`，生成时清洗为合法 Rust 标识符 `XBM`，副本 1088-1092）**未开放到 PF**，当前没有上报数据，未做 category 兼容。当 SE 把新内容开进 PF 并新增 category 位时，`DutyCategory` 的 serde 反序列化会在 `warp::body::json()` 阶段**整批拒绝**上报，且服务器端不会有任何「未插入」日志（特征性症状：插件上传批量失败 + 服务端零日志）。

修复方式参考当年给 V&C 迷宫加 `VariantAndCriterionDungeon` 的模式（先从插件日志 / Mongo 确认确切位值，再补一条链）：

- [13e6b2e](https://github.com/LittleNightmare/remote-party-finder/commit/13e6b2e866fbab641645ac51a19331fd13918c42) feat: Add support for V&C Dungeons
- [4d73b1c](https://github.com/LittleNightmare/remote-party-finder/commit/4d73b1cc717c2175956620c66102702759f64504) add missing VariantAndCriterionDungeonFinder

改动清单：`listing.rs` 的 `DutyCategory` 加显式变体 + `from_u32`/`as_u32`/`pf_category()`，`PartyFinderCategory` 加变体 + `ALL` + `as_str` + 四语名；`web/api.rs` 的 category 字符串映射表；`web/v2/id_inventory.rs` 的 `CATEGORY_IDS`（同步 `api_v2_contract.rs` 的 `id_inventory_is_stable` 断言）。网页分类下拉遍历 `ALL`，自动生效。

## Contributors
<a href="https://github.com/LittleNightmare/remote-party-finder/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=LittleNightmare/remote-party-finder" />
</a>

Made with [contrib.rocks](https://contrib.rocks).
