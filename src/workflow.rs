use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A single GitHub Actions workflow.
///
/// See: <https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions>
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: Trigger,
    #[serde(default)]
    pub permissions: Permissions,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Trigger {
    // A single "bare" event, like `on: push`.
    BareEvent(BareEvent),
    // Multiple "bare" events, like `on: [push, pull_request]`
    BareEvents(Vec<BareEvent>),
    // `schedule:` events.
    Schedule { schedule: Vec<Cron> },
    WorkflowCall { workflow_call: Option<WorkflowCall> },
    // "Rich" events, i.e. each event with its optional filters.
    Events(HashMap<BareEvent, Option<RichEvent>>),
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum BareEvent {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestComment,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    Push,
    RegistryPackage,
    Release,
    RepositoryDispatch,
    // NOTE: `schedule` is omitted, since it's never bare.
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cron {
    cron: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCall {
    inputs: HashMap<String, WorkflowCallInput>,
    outputs: HashMap<String, WorkflowCallOutput>,
    secrets: HashMap<String, WorkflowCallSecret>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallInput {
    description: Option<String>,
    // TODO: model `default`?
    #[serde(default)]
    required: bool,
    r#type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallOutput {
    description: Option<String>,
    value: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallSecret {
    description: Option<String>,
    required: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RichEvent {
    #[serde(default)]
    types: Vec<String>,

    // `push | pull_request | pull_request_target` only.
    #[serde(default)]
    branches: Vec<String>,

    // `push | pull_request | pull_request_target` only.
    #[serde(default)]
    branches_ignore: Vec<String>,

    // `push` only.
    #[serde(default)]
    tags: Vec<String>,

    // `push` only.
    #[serde(default)]
    tags_ignore: Vec<String>,

    // `push | pull_request | pull_request_target` only.
    #[serde(default)]
    paths: Vec<String>,

    // `push | pull_request | pull_request_target` only.
    #[serde(default)]
    paths_ignore: Vec<String>,
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Permissions {
    /// Whatever default permissions come from the workflow's `GITHUB_TOKEN`.
    #[default]
    Token,
    ReadAll,
    WriteAll,
    Explicit(ExplicitPermissions),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ExplicitPermissions {
    pub actions: Permission,
    pub checks: Permission,
    pub contents: Permission,
    pub deployments: Permission,
    pub id_token: Permission,
    pub issues: Permission,
    pub discussions: Permission,
    pub packages: Permission,
    pub pages: Permission,
    pub pull_requests: Permission,
    pub repository_projects: Permission,
    pub security_events: Permission,
    pub statuses: Permission,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Permission {
    Read,
    Write,
    None,
}
