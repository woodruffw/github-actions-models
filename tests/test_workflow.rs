use std::{env, path::Path};

use glomar_models::workflow::Workflow;

fn load_workflow(name: &str) -> Workflow {
    let workflow_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/sample-workflows")
        .join(name);
    let workflow_contents = std::fs::read_to_string(workflow_path).unwrap();
    serde_yaml::from_str(&workflow_contents).unwrap()
}

#[test]
fn test_load_all() {
    let sample_workflows = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/sample-workflows");

    for sample_action in std::fs::read_dir(&sample_workflows).unwrap() {
        let sample_workflow = sample_action.unwrap().path();
        let workflow_contents = std::fs::read_to_string(sample_workflow).unwrap();
        serde_yaml::from_str::<Workflow>(&workflow_contents).unwrap();
    }
}
