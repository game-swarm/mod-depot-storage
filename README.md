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

以下配置由 Engine 的 strict mod control plane 按 `mod.toml` 类型定义解码并注入 `DepotStorageConfig`。有效值来自 `mods.lock`；未知字段、错误类型和非法范围会阻止启动。

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

This crate depends on `swarm-engine-api` and `swarm-engine-plugin-sdk` version `0.1.0`. Until those crates are published to a registry, place the API repository beside `mods/` so the paths in `Cargo.toml` resolve:

```sh
mkdir -p swarm/mods
git clone <this-mod-repository-url> swarm/mods/depot-storage
# Fetch the reviewed API/SDK v0.1.0 source:
git clone --branch v0.1.0 https://github.com/game-swarm/engine-api swarm/engine-api
cd swarm/mods/depot-storage
cargo check
cargo test
```

To adopt a later API/SDK release, update both exact versions in `Cargo.toml` and the immutable API commit in `.github/workflows/ci.yml` together.
