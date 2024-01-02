use std::collections::HashMap;

use serde::Deserialize;

use crate::common::Env;

/// A GitHub Actions action definition.
///
/// See: <https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions>
/// and <https://json.schemastore.org/github-action.json>
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

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Input {
    pub description: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    pub description: String,
    // NOTE: not optional for composite actions, but this is not worth modeling.
    pub value: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Runs {
    JavaScript(JavaScript),
    Composite(Composite),
    Docker(Docker),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct JavaScript {
    // "node12" | "node16" | "node20"
    pub using: String,
    pub main: String,
    pub pre: Option<String>,
    // Defaults to `always()`
    pub pre_if: Option<String>,
    pub post: Option<String>,
    // Defaults to `always()`
    pub post_if: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Composite {
    // "composite"
    pub using: String,
    pub steps: Vec<Step>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Step {
    /// A step that runs a command in a shell.
    RunShell(RunShell),
    /// A step that uses another GitHub Action.
    UseAction(UseAction),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunShell {
    pub run: String,
    pub shell: String,
    pub name: Option<String>,
    pub id: Option<String>,
    pub r#if: Option<String>,
    #[serde(default)]
    pub env: Env,
    #[serde(default)]
    pub continue_on_error: bool,
    pub working_directory: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UseAction {
    pub uses: String,
    #[serde(default)]
    pub with: HashMap<String, String>,
    pub r#if: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Docker {
    // "docker"
    pub using: String,
    pub image: String,
    #[serde(default)]
    pub env: Env,
    pub entrypoint: Option<String>,
    pub pre_entrypoint: Option<String>,
    // Defaults to `always()`
    pub pre_if: Option<String>,
    pub post_entrypoint: Option<String>,
    // Defaults to `always()`
    pub post_if: Option<String>,
}
