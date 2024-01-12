//! "v2" Dependabot models.

use std::{
    collections::{HashMap, HashSet},
    default,
};

use serde::Deserialize;

use crate::common::SoV;

/// A `dependabot.yml` configuration file.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Dependabot {
    /// Invariant: `2`
    pub version: u64,
    #[serde(default)]
    pub enable_beta_ecosystems: bool,
    #[serde(default)]
    pub registries: HashMap<String, Registry>,
    pub updates: Vec<Update>,
}

/// Different registries known to Dependabot.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Registry {
    ComposerRepository {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    DockerRegistry {
        url: String,
        username: Option<String>,
        password: Option<String>,
        #[serde(default)]
        replaces_base: bool,
    },
    Git {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    HexOrganization {
        organization: String,
        key: Option<String>,
    },
    HexRepository {
        repo: Option<String>,
        url: String,
        auth_key: Option<String>,
        public_key_fingerprint: Option<String>,
    },
    MavenRepository {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    NpmRegistry {
        url: String,
        username: Option<String>,
        password: Option<String>,
        #[serde(default)]
        replaces_base: bool,
    },
    NugetFeed {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    PythonIndex {
        url: String,
        username: Option<String>,
        password: Option<String>,
        #[serde(default)]
        replaces_base: bool,
    },
    RubygemsServer {
        url: String,
        username: Option<String>,
        password: Option<String>,
        #[serde(default)]
        replaces_base: bool,
    },
    TerraformRegistry {
        url: String,
        token: Option<String>,
    },
}

/// A single `update` directive.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Update {
    #[serde(default)]
    pub allow: Vec<Allow>,
    #[serde(default)]
    pub assignees: HashSet<String>,
    pub commit_message: Option<CommitMessage>,
    pub directory: String,
    #[serde(default)]
    pub groups: HashMap<String, Group>,
    #[serde(default)]
    pub ignore: Vec<Ignore>,
    #[serde(default)]
    pub insecure_external_code_execution: AllowDeny,
    /// Labels to apply to this update group's pull requests.
    ///
    /// The default label is `dependencies`.
    #[serde(default = "default_labels")]
    pub labels: HashSet<String>,
    pub milestone: Option<u64>,
    /// The maximum number of pull requests to open at a time from this
    /// update group.
    ///
    /// The default maximum is 5.
    #[serde(default = "default_open_pull_requests_limit")]
    pub open_pull_requests_limit: u64,
    pub package_ecosystem: PackageEcosystem,
    // TODO: pull-request-branch-name
    #[serde(default)]
    pub rebase_strategy: RebaseStrategy,
    pub registries: Option<SoV<String>>,
    #[serde(default)]
    pub reviewers: HashSet<String>,
}

#[inline]
fn default_labels() -> HashSet<String> {
    HashSet::from(["dependencies".to_string()])
}

#[inline]
fn default_open_pull_requests_limit() -> u64 {
    // https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file#open-pull-requests-limit
    5
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Allow {
    pub dependency_name: Option<String>,
    pub dependency_type: Option<DependencyType>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DependencyType {
    Direct,
    Indirect,
    All,
    Production,
    Development,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CommitMessage {
    pub prefix: Option<String>,
    pub prefix_development: Option<String>,
    /// Invariant: `"scope"`
    pub include: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Group {
    /// This can only be [`DependencyType::Development`] or
    /// [`DependencyType::Production`].
    pub dependency_type: Option<DependencyType>,
    #[serde(default)]
    pub patterns: HashSet<String>,
    #[serde(default)]
    pub exclude_patterns: HashSet<String>,
    pub update_types: HashSet<UpdateType>,
}

#[derive(Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ignore {
    pub dependency_name: Option<String>,
    /// These are, inexplicably, not [`UpdateType`] variants.
    /// Instead, they're strings like `"version-update:semver-{major,minor,patch}"`.
    #[serde(default)]
    pub update_types: HashSet<String>,
    #[serde(default)]
    pub versions: HashSet<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AllowDeny {
    Allow,
    #[default]
    Deny,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PackageEcosystem {
    Bundler,
    Cargo,
    Composer,
    Docker,
    Elm,
    Gitsubmodule,
    GitHubActions,
    Gomod,
    Gradle,
    Maven,
    Mix,
    Npm,
    Nuget,
    Pip,
    Pub,
    Swift,
    Terraform,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RebaseStrategy {
    #[default]
    Auto,
    Disabled,
}
