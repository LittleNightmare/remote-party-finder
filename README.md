# Remote Party Finder
A fork version of Remote Party Finder from [Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)

this repo this prearing for CN server

Fork了[Anna Clemens](https://git.anna.lgbt/ascclemens/remote-party-finder)的Remote Party Finder

这个仓库是准备为了国服构建的，但我本人对server端的内容真的不熟。希望有人能帮帮我orz

## 关于显示FFXIV的特殊符号

参考这个[issue](https://github.com/LittleNightmare/remote-party-finder/issues/3)，下载对应的字体放在`server/assets`里

## Server 构建

Rust 版本由根目录的 `rust-toolchain.toml` 固定，本地开发和 GitHub Actions 使用同一个 stable 工具链。安装 [rustup](https://rustup.rs/) 后，在仓库中运行 Cargo 会自动选择该版本，无需修改其他项目的默认工具链。

```bash
cd server
cargo check --all-targets --locked
cargo test --locked
cargo build --release --locked
```

Windows 构建需要 Visual Studio 的 MSVC C++ 工具链和 Windows SDK。Linux 上生成与 CI 相同的静态 MUSL 产物，还需要 `musl-tools`：

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl --locked
```

运行服务仍需要配置 MongoDB；部署前应验证数据库连接和 API。

### 日志

服务端使用 Rust 生态常用的 `tracing` 日志系统。一般只需要在 `config.toml` 中填写一个等级：

```toml
[logging]
filter = "info"
```

等级从少到多为：

| 配置值 | 会显示的日志 |
|---|---|
| `error` | 错误 |
| `warn` | 警告和错误 |
| `info` | 一般运行信息、警告和错误 |
| `debug` | 调试信息以及以上等级 |
| `trace` | 最详细的调试信息 |
| `off` | 关闭日志 |

因此，日常使用通常只需要把 `filter` 改成 `info`、`debug` 或 `trace`。服务器 ID 超出
`validation.created_world_id_ranges` 的拒绝属于 `debug`，默认的 `info` 不会显示；其他招募
校验失败属于 `warn`，数据库和服务错误属于 `error`。

`filter` 这个名字来自 `tracing-subscriber` 的过滤器语法。它支持按模块进一步调整，但这是
高级用法，不影响上面的简单配置。规则用逗号分隔：单独的等级是全局默认值，
`模块=等级` 是该模块的例外规则，更具体的模块规则优先。例如：

```toml
# 依赖只显示警告和错误，本项目显示 debug 及以上日志
filter = "warn,remote_party_finder=debug"
```

这里的 `warn` 控制全局，`remote_party_finder=debug` 只为本项目开启调试日志。
`remote_party_finder` 是本项目的日志模块名；依赖库不属于这个模块，因此仍只输出警告和错误。

```toml
# 上面的配置基础上，单独隐藏服务器 ID 超范围日志
filter = "warn,remote_party_finder=debug,remote_party_finder::world_validation=off"
```

临时排查时也可以使用 `RUST_LOG` 环境变量覆盖 TOML 中的 `filter`，无需修改配置文件：

```bash
RUST_LOG=debug ./remote-party-finder ./config.toml
```

Windows PowerShell：

```powershell
$env:RUST_LOG = "debug"
./remote-party-finder.exe ./config.toml
```

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

## PF 招募的分类与任务编号（duty_type / category / duty）

招募上报里的三个字段来自客户端数据包（对应 FFXIVClientStructs 的 `CrossRealmListing`）：

| 字段 | 包内偏移 | 含义 |
|---|---|---|
| `category` | +0x1C u32 | 分类位标志，PF 页签 N → `1 << N`（1<<4=讨伐战、1<<13=深层迷宫…） |
| `duty` | +0x20 u16 | 任务编号，含义由 duty_type 决定 |
| `duty_type` | +0x22 u8 | 0=无（Other）、1=随机任务、2=具体任务 |

`duty` 的取值：

- `duty_type=2`：ContentFinderCondition 行号（副本/讨伐/团队等）；`duty_type=1`：ContentRoulette 行号（随机任务）；
- `duty_type=0` 且分类为金碟游乐场（1<<8）或深层迷宫（1<<13）：**LFGExtensionContent** 表的「行号 + 1」。这张表由 `csharp/SourceGenerator` 生成到 [`server/src/ffxiv/lfg_extension_content.rs`](server/src/ffxiv/lfg_extension_content.rs)，校验和网页显示都查它；
- `duty_type=0` 的其他分类：命运/寻宝按区域/宝图行号，怪物狩猎的编号含义未解析（显示固定名「怪物狩猎」），0 表示未指定。

**`duty = 0` 且分类为金碟/深层迷宫是合法组合**：客户端按内容匹配 LFGExtensionContent 失败（如内容未解锁）时只清零 duty、保留分类，网页显示为「深层迷宫」「金碟游乐场」通用名。

### 这套规则是怎么得来的（IDA 复现路径）

1. `AgentLookingForGroup.RequestCategoryListings` —— 里面用 `1 << category` 生成分类位标志；
2. `AgentLookingForGroup.PopulateListingData` —— 读网络包里的 listing 结构体，给出各字段偏移；
3. 选择写入函数（无名）：在 `AgentLookingForGroup` / `AddonLookingForGroupCondition` 的符号段里，找"对 ContentType 行号 1/2/3/4/5/6/8/9/19/21/30/39 做 switch、生成 `1 << N` 位标志"的那个。它枚举了全部合法组合：category 位来自所选内容的 ContentType 行（CT19→金碟、CT21/39→深层迷宫、CT30→V&C，高难度标志→64）；普通页签的 duty 包为 `(类型字节 << 16) | id`，金碟/深层迷宫页签写 LFGExtensionContent「行号 + 1」（深层迷宫 29–34 与当前表核对一致）；
4. 可用性检查函数（无名）：特征是"对 19/21/39 做 switch，分别用组 3、组 4 扫 LFGExtensionContent"。服务端「分类→分组」映射镜像它；
5. 行扫描函数（无名）：找引用 LFGExtensionContent（sheet 938）行查找、并循环读 `+0/+6/+7` 的函数——按内容匹配，命中写「行号 + 1」，失败清零；
6. sheet id 以客户端 `Component::Exd::SheetsEnum` 的名字为准（7.5：938 = LFGExtensionContent、1334 = DeepDungeonResident），id 本身随版本可能调整。

交叉验证：FFXIVClientStructs 的 [PartyFinderPackets.cs](https://github.com/goatcorp/FFXIVClientStructs/blob/main/FFXIVClientStructs/FFXIV/Client/Game/Network/PartyFinderPackets.cs)（`+0x1C Category`、`+0x20 Duty`、`+0x22 DutyType`，注释明确 Duty 可能是 LFGExtensionContent RowId）。

### 游戏更新后怎么快速再确认

1. **通常什么都不用改**：重跑生成器（`cd csharp/SourceGenerator && dotnet run -c Debug`，输出路径硬编码到 `server/src/ffxiv`），diff `lfg_extension_content.rs` —— 新增的金碟/深层迷宫条目自动进表、自动通过校验（例：7.5 新增的值 35 = 魔光键影）；
2. **出现新的 `unknown type/category/duty` WARN**：按上面第 1–6 步重新定位客户端逻辑（用符号名搜，别用旧地址），判断是编码变化还是新分类；只是新条目的话重跑生成器即可；
3. **怀疑行布局变化**：生成器运行时会把每一行的 `id/group/type` 打到控制台，与已知锚点比对（值 11 = 机遇任务/Addon 2308、值 29 = 死者宫殿、值 33/34 = 悲寂深想/歼灭战）即可发现偏移或分组语义是否变了；
4. `cargo test` 本机跑不了时看 CI；生成器编译运行需要能写 `obj/` 目录的权限。

相关：未来新增 category 位时的兼容见下方「已知问题」的 TODO 小节。

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
