use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A GitHub Actions action definition.
///
/// See: <https://docs.github.com/en/actions/creating-actions/metadata-syntax-for-github-actions>
/// and <https://json.schemastore.org/github-action.json>
#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Input {
    pub description: String,
    pub required: Option<bool>,
    pub default: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    pub description: String,
    // NOTE: not optional for composite actions, but this is not worth modeling.
    pub value: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Runs {
    JavaScript(JavaScript),
    Composite(Composite),
    Docker(Docker),
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Composite {
    // "composite"
    pub using: String,
    pub steps: Vec<Step>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Step {
    /// A step that runs a command in a shell.
    RunShell(RunShell),
    /// A step that uses another GitHub Action.
    UseAction(UseAction),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunShell {
    pub run: String,
    pub shell: String,
    pub name: Option<String>,
    pub id: Option<String>,
    pub r#if: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, EnvValue>,
    #[serde(default)]
    pub continue_on_error: bool,
    pub working_directory: Option<String>,
}

/// Environment variable values are always strings, but GitHub Actions
/// allows users to configure them as various native YAML types before
/// internal stringification.
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum EnvValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl ToString for EnvValue {
    fn to_string(&self) -> String {
        match self {
            Self::String(s) => s.clone(),
            Self::Number(n) => n.to_string(),
            Self::Boolean(b) => b.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UseAction {
    pub uses: String,
    #[serde(default)]
    pub with: HashMap<String, String>,
    pub r#if: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Docker {
    // "docker"
    pub using: String,
    pub image: String,
    #[serde(default)]
    pub env: HashMap<String, EnvValue>,
    pub entrypoint: Option<String>,
    pub pre_entrypoint: Option<String>,
    // Defaults to `always()`
    pub pre_if: Option<String>,
    pub post_entrypoint: Option<String>,
    // Defaults to `always()`
    pub post_if: Option<String>,
}
