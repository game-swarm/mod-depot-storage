# depot-storage

前线仓库（Forward Depot）物流模组。提供本地化维修和资源供给。

## 职责

- Forward Depot 实体：独立 Structure 类型，带本地资源存储
- 维修服务：drone 移动到 Depot range=1 内 → 消耗 Depot 本地资源 → 降低 drone age
- 队列管理：每个 Depot 有 repair_capacity / tick，drone 按确定性队列排序
- 相邻格只有 6 个 → 大量 drone 需要排队，形成物流拥挤决策
- 资源不足时本 tick 停止维修
- 没有全局 repair cap/cost——只受物理范围、设施容量、队列限制

## 依赖

- bevy

## 配置

以下配置由 Engine 按 `mod.toml` schema 严格验证 `world.toml [mods.depot-storage]` 并写入 replay config identity；`mods.lock` 不保存 gameplay config。native register context 当前保持 defaults-only parity，并把四个 versioned defaults 注入 `DepotStorageConfig`。未知字段、错误类型和非法范围会阻止启动。

mod.toml:
```toml
[config]
repair_range = { type = "u32", default = 1 }
repair_capacity = { type = "u32", default = 5 }
depot_hits = { type = "u32", default = 5000 }
depot_capacity = { type = "u32", default = 10000 }
```

## 资源

- 消耗 Depot 本地存储的资源维修 drone
- 资源通过 Transfer 指令由 drone 供给

## Standalone Development

This crate pins `swarm-engine-api` and `swarm-engine-plugin-sdk` to canonical source `https://github.com/game-swarm/engine-api.git`, exact version `0.1.0`, and identical full revision `0d97444af0c8f8c563bbe58837a4fdf8753630cf`. Cargo fetches both crates directly; no sibling API checkout is required.

```sh
cargo check
cargo test
```

To adopt a later API/SDK release, update both canonical URLs, both exact versions, and both full Git revisions in `Cargo.toml` together, then regenerate `Cargo.lock` and verify both packages resolve to the same commit.
