# lunar-interface

**Universal Multi-Layer Static Contract Protocol Suite & Core Topography Engine**

`lunar-interface` is the zero-dependency, highly-cohesive core library of the LunarAST ecosystem. It serves as the single source of truth for contract data structures, serialization formats, and topography alignment algorithms across both the `lunar` CLI and read-only distribution layers (`lunar-serve` and `lunar-gateway`).

---

## 🏛️ Architectural Role: Pure Structural Alignment

In distributed and microservice systems, **API semantic drift** often causes runtime failures that are expensive to diagnose. LunarAST solves this by extracting static facts at compile/build time. 

`lunar-interface` is strictly decoupled from CLI-specific operations (I/O, S3 SDKs, cryptography, terminal prompts) and HTTP-specific logic. This allows it to:
1. **Compile in seconds**: It has zero heavy third-party dependencies, allowing ultra-fast local compilation.
2. **WASM Native**: It compiles seamlessly to `wasm32-wasip2`, enabling edge-native, serverless execution inside `lunar-gateway`.
3. **Cohesive Schemas**: It mathematically ensures that CLI contract builders and server contract consumers share identical struct definitions.

---

## 📂 Core Data Models & Schemas

### 1. Physical Facts (`RouteSegment` & `RouteEntry`)
Represents the structural AST facts extracted from source code by AST adapters (e.g., Axum route parse trees).

*   `RouteSegment`: Represents a path unit (`literal`, `parameter`, or `wildcard`).
*   `RouteEntry`: Represents an endpoint containing HTTP method, segments, source mapping (file and line), and extraction metadata.
*   `ActualJson`: The read-only static facts container (excluded from Git).

### 2. Intent Overlay (`InterfacesYml` & `InterfaceItem`)
Represents the human-declared, Git-tracked developer intent contract. Used to overlay dynamic dependencies, route descriptions, or override extraction limits.

### 3. Compiled Topography (`LunarMap`)
Represents the entire ecosystem call-graph, alignments, and architectural anomalies.
*   `AlignmentEntry`: The mathematical alignment result of a client-consumed port to a server-exposed port (`Aligned`, `MethodMismatch`, `ParamNameMismatch`, or `Orphaned`).
*   `AggregatedEdge`: The summarized logical call edge between two microservice nodes, displaying call counts, warning states, and port maps.
*   `Anomalies`: Architectural risks computed from static facts:
    *   `unusedEndpoints`: Exposed endpoints with zero callers.
    *   `orphanedConsumers`: Consumed dependencies pointing to non-existent target routes.

---

## ⚙️ Core Topography Engine

The library exports the central algorithm:
```rust
pub fn generate_lunar_map(
    project_actuals: &HashMap<String, ActualJson>,
    scan_statuses: &HashMap<String, String>,
    project_paths: &HashMap<String, String>,
) -> LunarMap;
```

This engine:
1. Performs deterministic key-based matching of consumer AST structures against exposure AST structures.
2. Computes warn/risk status for every alignment without relying on blurry neural-network vector models.
3. Automatically embeds the **discovered workspace path (`path`)** of each project inside the map node, allowing stateless serving layers to dynamically locate and serve source code on-demand.

---

## 🚀 Integration Guide

Add this to your Cargo workspace members:

```toml
# In your Workspace Cargo.toml
[workspace]
members = [
    "lunar-interface",
    "lunar",
    "lunar-serve"
]

# In your member Cargo.toml (e.g., lunar-serve)
[dependencies]
lunar-interface = { path = "../lunar-interface" }
```

---

## 📜 License

All protocols and components in the LunarAST ecosystem are licensed under **Apache-2.0**.
