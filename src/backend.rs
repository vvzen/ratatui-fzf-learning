use std::collections::HashMap;

pub fn get_projects() -> Vec<String> {
    vec![
        "some_very_long_project_name".to_string(),
        "some_other_long_project_name".to_string(),
        "project_001".to_string(),
        "project_002".to_string(),
        "man_vs_bee".to_string(),
        "pipeline_testing_2022_2".to_string(),
        "asset_library_2024".to_string(),
        "asset_library_2023".to_string(),
        "rt_sandbox_2024".to_string(),
        "rnd_sandbox_2024".to_string(),
    ]
}

pub fn get_sequences(project_name: &str) -> Vec<String> {
    let mut projects_map: HashMap<&str, Vec<&str>> = HashMap::new();

    projects_map.insert("some_very_long_project_name", vec!["seq001", "seq002"]);
    projects_map.insert("project_001", vec!["seq002", "seq003"]);
    projects_map.insert("project_002", vec!["seq001", "seq002"]);

    let empty = Vec::new();
    let sequences = projects_map
        .get(project_name)
        .unwrap_or(&empty)
        .iter()
        .map(|&s| s.to_string())
        .collect::<Vec<String>>();

    sequences
}
