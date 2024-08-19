//! Data models for GitHub Actions workflow definitions.
//!
//! Resources:
//! * [Workflow syntax for GitHub Actions]
//! * [JSON Schema definition for workflows]
//!
//! [Workflow Syntax for GitHub Actions]: https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions>
//! [JSON Schema definition for workflows]: https://json.schemastore.org/github-workflow.json

use std::collections::HashMap;

use serde::Deserialize;

use crate::common::{BoE, Env, Permissions};

pub mod event;
pub mod job;

/// A single GitHub Actions workflow.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    pub name: Option<String>,
    pub run_name: Option<String>,
    pub on: Trigger,
    #[serde(default)]
    pub permissions: Permissions,
    #[serde(default)]
    pub env: Env,
    pub defaults: Option<Defaults>,
    pub concurrency: Option<Concurrency>,
    pub jobs: HashMap<String, Job>,
}

/// The triggering condition or conditions for a workflow.
///
/// Workflow triggers take three forms:
///
/// 1. A single webhook event name:
///
///     ```yaml
///     on: push
///     ```
/// 2. A list of webhook event names:
///
///     ```yaml
///     on: [push, fork]
///     ```
///
/// 3. A mapping of event names with (optional) configurations:
///
///     ```yaml
///     on:
///       push:
///         branches: [main]
///       pull_request:
///     ```
#[derive(Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Trigger {
    BareEvent(event::BareEvent),
    BareEvents(Vec<event::BareEvent>),
    Events(Box<event::Events>),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    pub run: Option<RunDefaults>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunDefaults {
    pub shell: Option<String>,
    pub working_directory: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Concurrency {
    pub group: String,
    #[serde(default)]
    pub cancel_in_progress: BoE,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Job {
    NormalJob(Box<job::NormalJob>),
    ReusableWorkflowCallJob(Box<job::ReusableWorkflowCallJob>),
}

impl Job {
    /// Returns the optional `name` field common to both reusable and normal
    /// job definitions.
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::NormalJob(job) => job.name.as_deref(),
            Self::ReusableWorkflowCallJob(job) => job.name.as_deref(),
        }
    }
}
