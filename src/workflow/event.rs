//! Workflow events.

use indexmap::IndexMap;
use serde::Deserialize;

/// "Bare" workflow event triggers.
///
/// These appear when a workflow is triggered with an event with no context,
/// e.g.:
///
/// ```yaml
/// on: push
/// ```
#[derive(Deserialize, PartialEq, Eq, Hash)]
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

/// Workflow event triggers, with bodies.
///
/// Like [`BareEvent`], but with per-event properties.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct Events {
    pub branch_protection_rule: OptionalBody<GenericEvent>,
    pub check_run: OptionalBody<GenericEvent>,
    pub check_suite: OptionalBody<GenericEvent>,
    // TODO: create + delete
    // TODO: deployment + deployment_status
    pub discussion: OptionalBody<GenericEvent>,
    pub discussion_comment: OptionalBody<GenericEvent>,
    // TODO: fork + gollum
    pub issue_comment: OptionalBody<GenericEvent>,
    pub issues: OptionalBody<GenericEvent>,
    pub label: OptionalBody<GenericEvent>,
    pub merge_group: OptionalBody<GenericEvent>,
    pub milestone: OptionalBody<GenericEvent>,
    // TODO: page_build
    pub project: OptionalBody<GenericEvent>,
    pub project_card: OptionalBody<GenericEvent>,
    pub project_column: OptionalBody<GenericEvent>,
    // TODO: public
    pub pull_request: OptionalBody<PullRequest>,
    pub pull_request_comment: OptionalBody<GenericEvent>,
    pub pull_request_review: OptionalBody<GenericEvent>,
    pub pull_request_review_comment: OptionalBody<GenericEvent>,
    // NOTE: `pull_request_target` appears to have the same trigger filters as `pull_request`.
    pub pull_request_target: OptionalBody<PullRequest>,
    pub push: OptionalBody<Push>,
    pub registry_package: OptionalBody<GenericEvent>,
    pub release: OptionalBody<GenericEvent>,
    pub repository_dispatch: OptionalBody<GenericEvent>,
    pub schedule: OptionalBody<Vec<Cron>>,
    // TODO: status
    pub watch: OptionalBody<GenericEvent>,
    pub workflow_call: OptionalBody<WorkflowCall>,
    // TODO: Custom type.
    pub workflow_dispatch: OptionalBody<WorkflowDispatch>,
    pub workflow_run: OptionalBody<WorkflowRun>,
}

/// A generic container type for distinguishing between
/// a missing key, an explicitly null key, and an explicit value `T`.
///
/// This is needed for modeling `on:` triggers, since GitHub distinguishes
/// between the non-presence of an event (no trigger) and the presence
/// of an empty event body (e.g. `pull_request:`), which means "trigger
/// with the defaults for this event type."
#[derive(Default)]
pub enum OptionalBody<T> {
    Default,
    #[default]
    Missing,
    Body(T),
}

impl<'de, T> Deserialize<'de> for OptionalBody<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T> From<Option<T>> for OptionalBody<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => OptionalBody::Body(v),
            None => OptionalBody::Default,
        }
    }
}

/// A generic event trigger body.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenericEvent {
    #[serde(default, deserialize_with = "crate::common::scalar_or_vector")]
    pub types: Vec<String>,
}

/// The body of a `pull_request` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PullRequest {
    #[serde(default)]
    pub types: Vec<String>,

    #[serde(flatten)]
    pub branch_filters: Option<BranchFilters>,

    #[serde(flatten)]
    pub path_filters: Option<PathFilters>,
}

/// The body of a `push` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Push {
    #[serde(flatten)]
    pub branch_filters: Option<BranchFilters>,

    #[serde(flatten)]
    pub path_filters: Option<PathFilters>,

    #[serde(flatten)]
    pub tag_filters: Option<TagFilters>,
}

/// The body of a `cron` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Cron {
    pub cron: String,
}

/// The body of a `workflow_call` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCall {
    #[serde(default)]
    pub inputs: IndexMap<String, WorkflowCallInput>,
    #[serde(default)]
    pub outputs: IndexMap<String, WorkflowCallOutput>,
    #[serde(default)]
    pub secrets: IndexMap<String, Option<WorkflowCallSecret>>,
}

/// A single input in a `workflow_call` event trigger body.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallInput {
    pub description: Option<String>,
    // TODO: model `default`?
    #[serde(default)]
    pub required: bool,
    pub r#type: String,
}

/// A single output in a `workflow_call` event trigger body.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallOutput {
    pub description: Option<String>,
    pub value: String,
}

/// A single secret in a `workflow_call` event trigger body.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallSecret {
    pub description: Option<String>,
    pub required: bool,
}

/// The body of a `workflow_dispatch` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDispatch {
    #[serde(default)]
    pub inputs: IndexMap<String, WorkflowDispatchInput>, // TODO: WorkflowDispatchInput
}

/// A single input in a `workflow_dispatch` event trigger body.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDispatchInput {
    pub description: Option<String>,
    // TODO: model `default`?
    #[serde(default)]
    pub required: bool,
    // TODO: Model as boolean, choice, number, environment, string; default is string.
    pub r#type: Option<String>,
    // Only present when `type` is `choice`.
    #[serde(default)]
    pub options: Vec<String>,
}

/// The body of a `workflow_run` event trigger.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowRun {
    pub workflows: Vec<String>,
    #[serde(default)]
    pub types: Vec<String>,
    #[serde(flatten)]
    pub branch_filters: Option<BranchFilters>,
}

/// Branch filtering variants for event trigger bodies.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BranchFilters {
    Branches(Vec<String>),
    BranchesIgnore(Vec<String>),
}

/// Tag filtering variants for event trigger bodies.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TagFilters {
    Tags(Vec<String>),
    TagsIgnore(Vec<String>),
}

/// Path filtering variants for event trigger bodies.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PathFilters {
    Paths(Vec<String>),
    PathsIgnore(Vec<String>),
}
