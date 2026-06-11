# lunar-interface

**通用多层静态契约协议套件与核心拓扑引擎**

`lunar-interface` 是 LunarAST 生态中**零依赖、高内聚**的核心库。它作为契约数据结构、序列化格式以及拓扑对齐算法的**单一真理源**，同时服务于 `lunar` 命令行工具与所有只读分发层（`lunar-serve` 和 `lunar-gateway`）。

---

## 🏛️ 架构定位：纯结构对齐

在分布式与微服务系统中，**API 语义漂移**往往会导致**诊断成本极高**的运行时故障。LunarAST 通过在编译/构建阶段提取静态事实从根源解决了这个问题。

`lunar-interface` 与所有 CLI 专属操作（输入输出、S3 软件开发工具包、密码学、终端交互提示）以及 HTTP 专属逻辑**严格解耦**。这让它具备以下特性：
1.  **秒级编译**：无任何重量级第三方依赖，支持极速本地编译。
2.  **原生 WASM 支持**：可无缝编译为 `wasm32-wasip2` 目标，在 `lunar-gateway` 中实现边缘原生、无服务器执行。
3.  **统一一致的模式定义**：从数学层面保证 CLI 契约构建器与服务端契约消费者使用完全相同的结构体定义。

---

## 📂 核心数据模型与模式定义

### 1. 物理事实层（`RouteSegment` & `RouteEntry`）
代表由 AST 适配器从源码中提取的结构化抽象语法树事实（例如 Axum 路由解析树）。

*   `RouteSegment`：代表一个路径单元（`literal` 字面量、`parameter` 参数或 `wildcard` 通配符）。
*   `RouteEntry`：代表一个端点，包含 HTTP 方法、路径段、源码映射（文件名和行号）以及提取元数据。
*   `ActualJson`：只读静态事实容器（不纳入 Git 版本控制）。

### 2. 意图覆盖层（`InterfacesYml` & `InterfaceItem`）
代表由人类声明、纳入 Git 版本追踪的开发者意图契约。用于补充动态依赖、路由描述或覆盖提取限制。

### 3. 编译后拓扑层（`LunarMap`）
代表整个生态的调用图、对齐关系以及架构异常。
*   `AlignmentEntry`：客户端消费端口与服务端暴露端口的数学对齐结果（`Aligned` 已对齐、`MethodMismatch` 方法不匹配、`ParamNameMismatch` 参数名不匹配或 `Orphaned` 悬空）。
*   `AggregatedEdge`：两个微服务节点之间的汇总逻辑调用边，展示调用次数、警告状态和端口映射。
*   `Anomalies`：基于静态事实计算出的架构风险：
    *   `unusedEndpoints`：无任何调用者的暴露端点。
    *   `orphanedConsumers`：指向不存在目标路由的消费依赖。

---

## ⚙️ 核心拓扑引擎

本库导出核心算法：
```rust
pub fn generate_lunar_map(
    project_actuals: &HashMap<String, ActualJson>,
    scan_statuses: &HashMap<String, String>,
    project_paths: &HashMap<String, String>,
) -> LunarMap;
```

该引擎具备以下能力：
1.  对消费者 AST 结构与服务端暴露 AST 结构执行**基于键的确定性匹配**。
2.  无需依赖模糊的神经网络向量模型，即可计算出每个对齐关系的警告/风险状态。
3.  自动将每个项目的**发现工作区路径（`path`）**嵌入到地图节点中，让无状态分发层能够按需动态定位并提供源码服务。

---

## 🚀 集成指南

将以下内容添加到你的 Cargo 工作区配置中：

```toml
# 在工作区根目录的 Cargo.toml 中
[workspace]
members = [
    "lunar-interface",
    "lunar",
    "lunar-serve"
]

# 在成员项目的 Cargo.toml 中（例如 lunar-serve）
[dependencies]
lunar-interface = { path = "../lunar-interface" }
```

---

## 📜 许可证

LunarAST 生态中的所有协议与组件均采用 **Apache-2.0** 许可证。
