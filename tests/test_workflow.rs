use std::{env, path::Path};

use github_actions_models::{
    common::expr::{ExplicitExpr, LoE},
    workflow::{
        event::OptionalBody,
        job::{RunsOn, StepBody},
        Job, Trigger, Workflow,
    },
};

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

    for sample_action in std::fs::read_dir(sample_workflows).unwrap() {
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
    let Job::NormalJob(test_job) = test_job else {
        panic!("expected normal job");
    };

    assert_eq!(test_job.name, None);
    assert_eq!(
        test_job.runs_on,
        LoE::Literal(RunsOn::Target(vec!["ubuntu-latest".to_string()]))
    );
    assert_eq!(test_job.steps.len(), 3);

    let StepBody::Uses { uses, with } = &test_job.steps[0].body else {
        panic!("expected uses step");
    };
    assert_eq!(uses, "actions/checkout@v4.1.1");
    assert!(with.is_empty());

    let StepBody::Uses { uses, with } = &test_job.steps[1].body else {
        panic!("expected uses step");
    };
    assert_eq!(uses, "actions/setup-python@v5");
    assert_eq!(with["python-version"].to_string(), "${{ matrix.python }}");
    assert_eq!(with["cache"].to_string(), "pip");
    assert_eq!(with["cache-dependency-path"].to_string(), "pyproject.toml");

    let StepBody::Run {
        run,
        working_directory,
        shell,
        env,
    } = &test_job.steps[2].body
    else {
        panic!("expected run step");
    };
    assert_eq!(run, "make test PIP_AUDIT_EXTRA=test");
    assert!(working_directory.is_none());
    assert!(shell.is_none());
    assert!(env.is_empty());
}

#[test]
fn test_runs_on_expr() {
    let workflow = load_workflow("runs-on-expr.yml");

    let job = workflow.jobs.get("check-bats-version").unwrap();
    let Job::NormalJob(job) = job else { panic!() };

    assert_eq!(
        job.runs_on,
        LoE::Expr(ExplicitExpr::from_curly("${{ matrix.runner }}").unwrap())
    );
}
