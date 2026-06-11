use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------- Core Data Structures ----------

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteSegment {
    #[serde(rename = "type")]
    pub segment_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_constraint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteEntry {
    pub method: String,
    pub segments: Vec<RouteSegment>,
    pub source_file: String,
    pub line_number: u32,
    pub extraction_method: String,
    #[serde(default)]
    pub target_project: Option<String>,
}

impl RouteEntry {
    pub fn to_path(&self) -> String {
        let mut path = String::new();
        for seg in &self.segments {
            match seg.segment_type.as_str() {
                "literal" => { path.push('/'); path.push_str(seg.value.as_ref().unwrap()); }
                "parameter" => { path.push_str("/{"); path.push_str(seg.name.as_ref().unwrap()); path.push('}'); }
                "wildcard" => path.push_str("/*"),
                _ => {}
            }
        }
        path
    }

    pub fn display_path(&self) -> String {
        let mut path = String::new();
        for seg in &self.segments {
            match seg.segment_type.as_str() {
                "literal" => { path.push('/'); path.push_str(seg.value.as_ref().unwrap()); }
                "parameter" => { path.push_str("/:"); path.push_str(seg.name.as_ref().unwrap()); }
                "wildcard" => path.push_str("/*"),
                _ => {}
            }
        }
        path
    }

    pub fn structural_id(&self) -> String {
        let types: Vec<&str> = self.segments.iter().map(|s| s.segment_type.as_str()).collect();
        types.join(":")
    }

    pub fn get_aligned_parameter_names(&self, other: &RouteEntry) -> Vec<String> {
        let len = self.segments.len().min(other.segments.len());
        let mut names = Vec::new();
        for i in 0..len {
            let a = &self.segments[i];
            let b = &other.segments[i];
            if a.segment_type == "parameter" && b.segment_type == "parameter" {
                names.push(a.name.clone().unwrap_or_default());
            }
        }
        names
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActualJson {
    pub exposed: Vec<RouteEntry>,
    pub consumed: Vec<RouteEntry>,
    #[serde(default)]
    pub project_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InterfacesYml {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed: Option<Vec<InterfaceItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed: Option<Vec<InterfaceItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceItem {
    pub path: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "targetProject")]
    pub target_project: Option<String>,
}

// ---------- Diff & Alignment Logic ----------

#[derive(Debug, PartialEq)]
pub enum DiffResult {
    Added,
    Removed,
    MethodChanged { old_method: String, new_method: String },
    ParamNamesChanged { old_names: Vec<String>, new_names: Vec<String> },
    Unchanged,
}

pub fn compare_routes(old: &RouteEntry, new: &RouteEntry) -> DiffResult {
    if old.method != new.method {
        return DiffResult::MethodChanged { old_method: old.method.clone(), new_method: new.method.clone() };
    }
    let old_params = old.get_aligned_parameter_names(new);
    let new_params = new.get_aligned_parameter_names(old);
    if old_params != new_params {
        return DiffResult::ParamNamesChanged { old_names: old_params, new_names: new_params };
    }
    DiffResult::Unchanged
}

pub fn build_structural_index(routes: &[RouteEntry]) -> HashMap<String, Vec<RouteEntry>> {
    let mut index: HashMap<String, Vec<RouteEntry>> = HashMap::new();
    for r in routes { index.entry(r.structural_id()).or_default().push(r.clone()); }
    index
}

// ---------- Map Schema & Topography Generation ----------

#[derive(Debug, Serialize, Deserialize)]
pub struct LunarMapConfig {
    pub projects: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub project_type: String,
    pub sha: String,
    #[serde(rename = "scanStatus")]
    pub scan_status: String,
    pub interfaces: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlignmentEntry {
    #[serde(rename = "clientProject")]
    pub client_project: String,
    #[serde(rename = "serverProject")]
    pub server_project: String,
    pub path: String,
    pub method: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortConnection {
    pub path: String,
    pub method: String,
    pub status: String,
    #[serde(rename = "sourcePortIndex")]
    pub source_port_index: usize,
    #[serde(rename = "targetPortIndex")]
    pub target_port_index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregatedEdge {
    #[serde(rename = "clientProject")]
    pub client_project: String,
    #[serde(rename = "serverProject")]
    pub server_project: String,
    #[serde(rename = "callCount")]
    pub call_count: usize,
    pub status: String,
    pub paths: Vec<String>,
    pub ports: Vec<PortConnection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Anomalies {
    #[serde(rename = "unusedEndpoints")]
    pub unused_endpoints: Vec<AnomalyEndpoint>,
    #[serde(rename = "orphanedConsumers")]
    pub orphaned_consumers: Vec<AnomalyEndpoint>,
    #[serde(rename = "crossLayerViolations")]
    pub cross_layer_violations: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnomalyEndpoint {
    pub project: String,
    pub path: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LunarMap {
    pub version: String,
    pub projects: Vec<ProjectInfo>,
    pub alignments: Vec<AlignmentEntry>,
    #[serde(rename = "aggregatedEdges")]
    pub aggregated_edges: Vec<AggregatedEdge>,
    pub anomalies: Anomalies,
}

fn align_consumed_route(consumed: &RouteEntry, client_name: &str, project_map: &HashMap<String, &ActualJson>, scan_status_map: &HashMap<String, String>) -> (String, Option<String>) {
    let target = consumed.target_project.as_deref().unwrap_or("");
    let target_scan_failed = !target.is_empty() && scan_status_map.get(target).map_or(false, |s| s == "failed" || s == "stale");
    if target_scan_failed { return ("Unverified".to_string(), Some("Server project scan failed or stale".to_string())); }
    for (server_name, server_actual) in project_map {
        if server_name == client_name { continue; }
        if !target.is_empty() && target != *server_name { continue; }
        if let Some(server_route) = server_actual.exposed.iter().find(|r| r.structural_id() == consumed.structural_id()) {
            let result = compare_routes(consumed, server_route);
            let (status, warning) = match result {
                DiffResult::Unchanged => ("Aligned".to_string(), None),
                DiffResult::ParamNamesChanged { .. } => ("ParamNameMismatch".to_string(), None),
                DiffResult::MethodChanged { .. } => ("MethodMismatch".to_string(), None),
                _ => unreachable!(),
            };
            if scan_status_map.get(server_name).map_or(false, |s| s == "failed" || s == "stale") {
                return ("Unverified".to_string(), Some("Server project scan failed or stale".to_string()));
            }
            return (status, warning);
        }
    }
    if target.is_empty() { (String::new(), None) } else { ("Orphaned".to_string(), None) }
}

fn aggregate_status(statuses: &[String]) -> String {
    let priority = |s: &str| -> usize { match s { "MethodMismatch" => 0, "Orphaned" => 1, "ParamNameMismatch" => 2, "Aligned" => 3, _ => 4 } };
    statuses.iter().min_by_key(|s| priority(s)).cloned().unwrap_or_else(|| "Aligned".to_string())
}

fn detect_unused_endpoints(project_actuals: &HashMap<String, ActualJson>, alignments: &[AlignmentEntry]) -> Vec<AnomalyEndpoint> {
    let mut unused = Vec::new();
    for (name, actual) in project_actuals {
        if actual.project_type.as_deref() == Some("library") {
            continue;
        }
        for exposed in &actual.exposed {
            let consumed_by_any = alignments.iter().any(|a| a.server_project == *name && a.path == exposed.to_path() && a.method == exposed.method && a.status != "Orphaned");
            if !consumed_by_any { unused.push(AnomalyEndpoint { project: name.clone(), path: exposed.to_path(), method: exposed.method.clone() }); }
        }
    }
    unused
}

pub fn generate_lunar_map(
    project_actuals: &HashMap<String, ActualJson>,
    scan_statuses: &HashMap<String, String>,
    project_paths: &HashMap<String, String>,
) -> LunarMap {
    let mut projects = Vec::new();
    let mut alignments = Vec::new();
    for (name, actual) in project_actuals {
        let scan_status = scan_statuses.get(name).cloned().unwrap_or_else(|| "success".to_string());
        let project_type = actual.project_type.clone().unwrap_or_else(|| "mixed".to_string());
        let interfaces = serde_json::json!({
            "exposed": actual.exposed.iter().map(|r| serde_json::json!({"path": r.to_path(), "method": r.method})).collect::<Vec<_>>(),
            "consumed": actual.consumed.iter().map(|r| serde_json::json!({"path": r.to_path(), "method": r.method, "targetProject": r.target_project.as_deref().unwrap_or("unknown")})).collect::<Vec<_>>(),
        });
        let path = project_paths.get(name).cloned();
        projects.push(ProjectInfo { name: name.clone(), project_type, sha: "unknown".to_string(), scan_status, interfaces, path });
    }
    let project_map: HashMap<String, &ActualJson> = project_actuals.iter().map(|(k, v)| (k.clone(), v)).collect();
    let scan_status_map = scan_statuses.clone();
    for (client_name, actual) in project_actuals {
        for consumed in &actual.consumed {
            let (status, warning) = align_consumed_route(consumed, client_name, &project_map, &scan_status_map);
            if status.is_empty() { continue; }
            alignments.push(AlignmentEntry { client_project: client_name.clone(), server_project: consumed.target_project.as_deref().unwrap_or("unknown").to_string(), path: consumed.to_path(), method: consumed.method.clone(), status, warning });
        }
    }
    let mut edge_groups: HashMap<(String, String), Vec<&AlignmentEntry>> = HashMap::new();
    for entry in &alignments { edge_groups.entry((entry.client_project.clone(), entry.server_project.clone())).or_default().push(entry); }
    let aggregated_edges: Vec<AggregatedEdge> = edge_groups.into_iter().map(|((client, server), entries)| {
        let statuses: Vec<String> = entries.iter().map(|e| e.status.clone()).collect();
        let paths: Vec<String> = entries.iter().map(|e| format!("{} {}", e.method, e.path)).collect();
        let mut ports = Vec::new();
        if let (Some(ca), Some(sa)) = (project_actuals.get(&client), project_actuals.get(&server)) {
            for entry in &entries {
                let si = ca.consumed.iter().position(|c| c.to_path() == entry.path && c.method == entry.method);
                let ti = sa.exposed.iter().position(|e| e.to_path() == entry.path && e.method == entry.method);
                if let (Some(s), Some(t)) = (si, ti) { ports.push(PortConnection { path: entry.path.clone(), method: entry.method.clone(), status: entry.status.clone(), source_port_index: s, target_port_index: t }); }
            }
        }
        AggregatedEdge { client_project: client, server_project: server, call_count: entries.len(), status: aggregate_status(&statuses), paths, ports }
    }).collect();
    let unused_endpoints = detect_unused_endpoints(project_actuals, &alignments);
    let orphaned_consumers: Vec<AnomalyEndpoint> = alignments.iter().filter(|a| a.status == "Orphaned").map(|a| AnomalyEndpoint { project: a.client_project.clone(), path: a.path.clone(), method: a.method.clone() }).collect();
    LunarMap { version: "0.5.0".to_string(), projects, alignments, aggregated_edges, anomalies: Anomalies { unused_endpoints, orphaned_consumers, cross_layer_violations: vec![] } }
}

// ---------- Intent overlay merge ----------

pub fn merge_intent_into_actual(actual: &mut ActualJson, intent: &InterfacesYml) {
    if let Some(ref pt) = intent.project_type {
        actual.project_type = Some(pt.clone());
    }
    if let Some(ref intent_exposed) = intent.exposed {
        for intent_item in intent_exposed {
            if let Some(existing) = actual.exposed.iter_mut().find(|e| e.to_path() == intent_item.path && e.method == intent_item.method) {
                if intent_item.target_project.is_some() { existing.target_project = intent_item.target_project.clone(); }
            } else {
                let segments = parse_path_to_segments(&intent_item.path);
                actual.exposed.push(RouteEntry {
                    method: intent_item.method.clone(), segments,
                    source_file: "manual".to_string(), line_number: 0, extraction_method: "manual".to_string(), target_project: None,
                });
            }
        }
    }
    if let Some(ref intent_consumed) = intent.consumed {
        for intent_item in intent_consumed {
            if let Some(existing) = actual.consumed.iter_mut().find(|e| e.to_path() == intent_item.path && e.method == intent_item.method) {
                if intent_item.target_project.is_some() { existing.target_project = intent_item.target_project.clone(); }
            } else {
                let segments = parse_path_to_segments(&intent_item.path);
                actual.consumed.push(RouteEntry {
                    method: intent_item.method.clone(), segments,
                    source_file: "manual".to_string(), line_number: 0, extraction_method: "manual".to_string(), target_project: intent_item.target_project.clone(),
                });
            }
        }
    }
}

fn parse_path_to_segments(path: &str) -> Vec<RouteSegment> {
    let mut segments = Vec::new();
    for part in path.trim_matches('/').split('/') {
        if part.is_empty() { continue; }
        if part.starts_with(':') {
            segments.push(RouteSegment { segment_type: "parameter".to_string(), value: None, name: Some(part[1..].to_string()), raw_constraint: None });
        } else if part.starts_with('{') && part.ends_with('}') {
            let inner = &part[1..part.len()-1];
            let name = inner.split(':').next().unwrap_or(inner).to_string();
            segments.push(RouteSegment { segment_type: "parameter".to_string(), value: None, name: Some(name), raw_constraint: None });
        } else {
            segments.push(RouteSegment { segment_type: "literal".to_string(), value: Some(part.to_string()), name: None, raw_constraint: None });
        }
    }
    segments
}
