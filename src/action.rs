//! Data models for GitHub Actions action definitions.
//!
//! Resources:
//! * [Metadata syntax for GitHub Actions]
//! * [JSON Schema definition for GitHub Actions]
//!
//! [Metadata syntax for GitHub Actions]: https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions
//! [JSON Schema definition for GitHub Actions]: https://json.schemastore.org/github-action.json

use std::collections::HashMap;

use serde::Deserialize;

use crate::common::{BoE, Env};

/// A GitHub Actions action definition.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Action {
    pub name: String,
    pub author: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub inputs: HashMap<String, Input>,
    #[serde(default)]
    pub outputs: HashMap<String, Output>,
    pub runs: Runs,
}

/// An action input.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Input {
    pub description: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

/// An action output.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    pub description: String,
    // NOTE: not optional for composite actions, but this is not worth modeling.
    pub value: Option<String>,
}

/// An action `runs` definition.
///
/// A `runs` definition can be either a JavaScript action, a "composite" action
/// (made up of several constituent actions), or a Docker action.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Runs {
    JavaScript(JavaScript),
    Composite(Composite),
    Docker(Docker),
}

/// A `runs` definition for a JavaScript action.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JavaScript {
    /// The Node runtime to use for this action. This is one of:
    ///
    /// `"node12" | "node16" | "node20"`
    pub using: String,

    /// The action's entrypoint, as a JavaScript file.
    pub main: String,

    /// An optional script to run, before [`JavaScript::main`].
    pub pre: Option<String>,

    /// An optional expression that triggers [`JavaScript::pre`] if it evaluates to `true`.
    ///
    /// If not present, defaults to `always()`
    pub pre_if: Option<String>,

    /// An optional script to run, after [`JavaScript::main`].
    pub post: Option<String>,

    /// An optional expression that triggers [`JavaScript::post`] if it evaluates to `true`.
    ///
    /// If not present, defaults to `always()`
    pub post_if: Option<String>,
}

/// A `runs` definition for a composite action.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Composite {
    /// Invariant: `"composite"`
    pub using: String,
    /// The individual steps that make up this composite action.
    pub steps: Vec<Step>,
}

/// An individual composite action step.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Step {
    RunShell(RunShell),
    UseAction(UseAction),
}

/// A step that runs a command in a shell.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunShell {
    /// The command to run.
    pub run: String,

    /// The shell to run in.
    pub shell: String,

    /// An optional name for this step.
    pub name: Option<String>,

    /// An optional ID for this step.
    pub id: Option<String>,

    /// An optional expression that prevents this step from running unless it evaluates to `true`.
    pub r#if: Option<String>,

    /// An optional environment mapping for this step.
    #[serde(default)]
    pub env: Env,

    /// A an optional boolean or expression that, if `true`, prevents the job from failing when
    /// this step fails.
    #[serde(default)]
    pub continue_on_error: BoE,

    /// An optional working directory to run [`RunShell::run`] from.
    pub working_directory: Option<String>,
}

/// A step that uses another GitHub Action.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UseAction {
    /// The GitHub Action being used.
    pub uses: String,

    /// Any inputs to the action being used.
    #[serde(default)]
    pub with: HashMap<String, String>,

    /// An optional expression that prevents this step from running unless it evaluates to `true`.
    pub r#if: Option<String>,
}

/// A `runs` definition for a Docker action.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Docker {
    /// Invariant: `"docker"`
    pub using: String,

    /// The Docker image to use.
    pub image: String,

    /// An optional environment mapping for this step.
    #[serde(default)]
    pub env: Env,

    /// An optional Docker entrypoint, potentially overriding the image's
    /// default entrypoint.
    pub entrypoint: Option<String>,

    /// An optional "pre" entrypoint to run, before [`Docker::entrypoint`].
    pub pre_entrypoint: Option<String>,

    /// An optional expression that triggers [`Docker::pre_entrypoint`] if it evaluates to `true`.
    ///
    /// If not present, defaults to `always()`
    pub pre_if: Option<String>,

    /// An optional "post" entrypoint to run, after [`Docker::entrypoint`] or the default
    /// entrypoint.
    pub post_entrypoint: Option<String>,

    /// An optional expression that triggers [`Docker::post_entrypoint`] if it evaluates to `true`.
    ///
    /// If not present, defaults to `always()`
    pub post_if: Option<String>,
}
