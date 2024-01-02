use std::{env, path::Path};

use glomar_models::workflow::{event::OptionalBody, job::RunsOn, Job, Trigger, Workflow};

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

#[test]
fn test_pip_audit_ci() {
    let workflow = load_workflow("pip-audit-ci.yml");

    assert!(
        matches!(workflow.on, Trigger::Events(events) if matches!(events.pull_request, OptionalBody::Default))
    );

    let test_job = &workflow.jobs["test"];
    if let Job::NormalJob(test_job) = test_job {
        assert_eq!(test_job.name, None);
        assert_eq!(
            test_job.runs_on,
            RunsOn::Target(String::from("ubuntu-latest").into())
        );
    } else {
        panic!("oops");
    }
}
