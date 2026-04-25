# Remote Party Finder
A fork version of Remote Party Finder from [Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)

this repo this prearing for CN server

Fork了[Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)的Remote Party Finder

这个仓库是准备为了国服构建的，但我本人对server端的内容真的不熟。希望有人能帮帮我orz

## 关于显示FFXIV的特殊符号

参考这个[issue](https://github.com/LittleNightmare/remote-party-finder/issues/3)，下载对应的字体放在`server/assets`里

## API
已经提供了 API，使用方式请见 [API 使用文档](https://github.com/LittleNightmare/remote-party-finder/wiki/API-Usage)。

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

## Contributors
<a href="https://github.com/LittleNightmare/remote-party-finder/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=LittleNightmare/remote-party-finder" />
</a>

Made with [contrib.rocks](https://contrib.rocks).
