//! Workflow jobs.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use crate::common::expr::{BoE, LoE};
use crate::common::{Env, Permissions};

use super::{Concurrency, Defaults};

/// A "normal" GitHub Actions workflow job, i.e. a job composed of one
/// or more steps on a runner.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NormalJob {
    pub name: Option<String>,
    #[serde(default)]
    pub permissions: Permissions,
    #[serde(default, deserialize_with = "crate::common::scalar_or_vector")]
    pub needs: Vec<String>,
    pub r#if: Option<String>,
    pub runs_on: RunsOn,
    pub environment: Option<DeploymentEnvironment>,
    pub concurrency: Option<Concurrency>,
    #[serde(default)]
    pub outputs: HashMap<String, String>,
    #[serde(default)]
    pub env: Env,
    pub defaults: Option<Defaults>,
    pub steps: Vec<Step>,
    pub timeout_minutes: Option<LoE<u64>>,
    pub strategy: Option<Strategy>,
    #[serde(default)]
    pub continue_on_error: BoE,
    pub container: Option<Container>,
    #[serde(default)]
    pub services: HashMap<String, Container>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum RunsOn {
    #[serde(deserialize_with = "crate::common::scalar_or_vector")]
    Target(Vec<String>),
    Group {
        group: String,
        #[serde(deserialize_with = "crate::common::scalar_or_vector")]
        labels: Vec<String>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum DeploymentEnvironment {
    Name(String),
    NameURL { name: String, url: Option<String> },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    pub id: Option<String>,
    pub r#if: Option<String>,
    pub name: Option<String>,
    pub timeout_minutes: Option<u64>,
    #[serde(default)]
    pub continue_on_error: BoE,
    #[serde(flatten)]
    pub body: StepBody,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum StepBody {
    Uses {
        uses: String,
        #[serde(default)]
        with: Env,
    },
    Run {
        #[serde(deserialize_with = "crate::common::bool_is_string")]
        run: String,
        working_directory: Option<String>,
        shell: Option<String>,
        #[serde(default)]
        env: Env,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Strategy {
    pub matrix: LoE<Matrix>,
    pub fail_fast: Option<BoE>,
    pub max_parallel: Option<u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Matrix {
    #[serde(default)]
    pub include: LoE<Vec<HashMap<String, Value>>>,
    #[serde(default)]
    pub exclude: LoE<Vec<HashMap<String, Value>>>,
    #[serde(flatten)]
    pub dimensions: LoE<HashMap<String, LoE<Vec<Value>>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Container {
    Name(String),
    Container {
        image: String,
        credentials: Option<DockerCredentials>,
        #[serde(default)]
        env: Env,
        // TODO: model `ports`?
        #[serde(default)]
        volumes: Vec<String>,
        options: Option<String>,
    },
}

#[derive(Deserialize)]
pub struct DockerCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ReusableWorkflowCallJob {
    pub name: Option<String>,
    #[serde(default)]
    pub permissions: Permissions,
    #[serde(default, deserialize_with = "crate::common::scalar_or_vector")]
    pub needs: Vec<String>,
    pub r#if: Option<String>,
    pub uses: String,
    #[serde(default)]
    pub with: Env,
    pub secrets: Option<Secrets>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Secrets {
    Inherit,
    #[serde(untagged)]
    Env(#[serde(default)] Env),
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{expr::LoE, EnvValue},
        workflow::job::{Matrix, Secrets},
    };

    use super::Strategy;

    #[test]
    fn test_secrets() {
        assert_eq!(
            serde_yaml::from_str::<Secrets>("inherit").unwrap(),
            Secrets::Inherit
        );

        let secrets = "foo-secret: bar";
        let Secrets::Env(secrets) = serde_yaml::from_str::<Secrets>(secrets).unwrap() else {
            panic!("unexpected secrets variant");
        };
        assert_eq!(secrets["foo-secret"], EnvValue::String("bar".into()));
    }

    #[test]
    fn test_strategy_matrix_expressions() {
        let strategy = "matrix: ${{ 'foo' }}";
        let Strategy {
            matrix: LoE::Expr(expr),
            ..
        } = serde_yaml::from_str::<Strategy>(strategy).unwrap()
        else {
            panic!("unexpected matrix variant");
        };

        assert_eq!(expr.as_curly(), "${{ 'foo' }}");

        let strategy = r#"
matrix:
  foo: ${{ 'foo' }}
"#;

        let Strategy {
            matrix:
                LoE::Literal(Matrix {
                    include: _,
                    exclude: _,
                    dimensions: LoE::Literal(dims),
                }),
            ..
        } = serde_yaml::from_str::<Strategy>(strategy).unwrap()
        else {
            panic!("unexpected matrix variant");
        };

        assert!(matches!(dims.get("foo"), Some(LoE::Expr(_))));
    }
}
