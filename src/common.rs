//! Shared models and utilities.

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Deserializer, Serialize};

pub mod expr;

/// `permissions` for a workflow, job, or step.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Permissions {
    /// Base, i.e. blanket permissions.
    Base(BasePermission),
    /// Fine-grained permissions.
    ///
    /// These are modeled with an open-ended mapping rather than a structure
    /// to make iteration over all defined permissions easier.
    Explicit(HashMap<String, Permission>),
}

impl Default for Permissions {
    fn default() -> Self {
        Self::Base(BasePermission::Default)
    }
}

/// "Base" permissions, where all individual permissions are configured
/// with a blanket setting.
#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum BasePermission {
    /// Whatever default permissions come from the workflow's `GITHUB_TOKEN`.
    #[default]
    Default,
    /// "Read" access to all resources.
    ReadAll,
    /// "Write" access to all resources (implies read).
    WriteAll,
}

/// A singular permission setting.
#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    /// Read access.
    Read,

    /// Write access.
    Write,

    /// No access.
    #[default]
    None,
}

/// An environment mapping.
pub type Env = HashMap<String, EnvValue>;

/// Environment variable values are always strings, but GitHub Actions
/// allows users to configure them as various native YAML types before
/// internal stringification.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum EnvValue {
    // Missing values are empty strings.
    #[serde(deserialize_with = "null_to_default")]
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Display for EnvValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(b) => write!(f, "{b}"),
        }
    }
}

/// A "scalar or vector" type, for places in GitHub Actions where a
/// key can have either a scalar value or an array of values.
///
/// This only appears internally, as an intermediate type for `scalar_or_vector`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum SoV<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<SoV<T>> for Vec<T> {
    fn from(val: SoV<T>) -> Vec<T> {
        match val {
            SoV::One(v) => vec![v],
            SoV::Many(vs) => vs,
        }
    }
}

pub(crate) fn scalar_or_vector<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    SoV::deserialize(de).map(Into::into)
}

/// A bool or string. This is useful for cases where GitHub Actions contextually
/// reinterprets a YAML boolean as a string, e.g. `run: true` really means
/// `run: 'true'`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum BoS {
    Bool(bool),
    String(String),
}

impl From<BoS> for String {
    fn from(value: BoS) -> Self {
        match value {
            BoS::Bool(b) => b.to_string(),
            BoS::String(s) => s,
        }
    }
}

/// An `if:` condition in a job or action definition.
///
/// These are either booleans or bare (i.e. non-curly) expressions.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum If {
    Bool(bool),
    // NOTE: expressions in conditions are "bare", not curly. So we don't
    // use `LoE`/`BoE` here.
    Expr(String),
}

pub(crate) fn bool_is_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoS::deserialize(de).map(Into::into)
}

fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let key = Option::<T>::deserialize(de)?;
    Ok(key.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::common::{BasePermission, Env, EnvValue, Permission};

    use super::Permissions;

    #[test]
    fn test_permissions() {
        assert_eq!(
            serde_yaml::from_str::<Permissions>("read-all").unwrap(),
            Permissions::Base(BasePermission::ReadAll)
        );

        let perm = "security-events: write";
        assert_eq!(
            serde_yaml::from_str::<Permissions>(perm).unwrap(),
            Permissions::Explicit(HashMap::from([(
                "security-events".into(),
                Permission::Write
            )]))
        );
    }

    #[test]
    fn test_env_empty_value() {
        let env = "foo:";
        assert_eq!(
            serde_yaml::from_str::<Env>(env).unwrap()["foo"],
            EnvValue::String("".into())
        );
    }
}
