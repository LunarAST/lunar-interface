use lunar_interface::{Ci144Config, generate_lunar_map, ActualJson};
use std::collections::HashMap;

#[test]
fn test_ci144_config_deserialization() {
    let yaml_str = r#"
project: cellrix
target: cellrix
semanticNodes:
  - name: cin7_snapshot
    type: snap
    description: snapshot desc
actions:
  - name: action_test
    fn: execute_action
    description: action desc
    payloadFields: ["payload1"]
"#;
    let config: Result<Ci144Config, _> = serde_yaml::from_str(yaml_str);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.project, "cellrix");
    assert_eq!(config.target, "cellrix");
    assert_eq!(config.semantic_nodes.unwrap()[0].name, "cin7_snapshot");
}

#[test]
fn test_generate_lunar_map_with_hashes() {
    let mut project_actuals = HashMap::new();
    let mut scan_statuses = HashMap::new();
    let mut project_paths = HashMap::new();
    let mut project_hashes = HashMap::new();

    project_actuals.insert("cellrix".to_string(), ActualJson {
        exposed: vec![],
        consumed: vec![],
        project_type: Some("library".to_string()),
    });
    scan_statuses.insert("cellrix".to_string(), "success".to_string());
    project_paths.insert("cellrix".to_string(), "/opt/cellrix".to_string());
    
    let mut cellrix_hashes = HashMap::new();
    cellrix_hashes.insert("cellrix".to_string(), "3f4a2b1c".to_string());
    project_hashes.insert("cellrix".to_string(), cellrix_hashes);

    let map = generate_lunar_map(&project_actuals, &scan_statuses, &project_paths, &project_hashes);
    assert_eq!(map.version, "0.5.0");
    assert_eq!(map.projects.len(), 1);
    assert_eq!(map.projects[0].name, "cellrix");
    assert_eq!(map.projects[0].path.as_deref(), Some("/opt/cellrix"));
    
    let hashes = map.projects[0].hashes.as_ref().unwrap();
    assert_eq!(hashes.get("cellrix").unwrap(), "3f4a2b1c");
}
