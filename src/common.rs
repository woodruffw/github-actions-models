//! Shared models and utilities.

use std::collections::HashMap;

use serde::Deserialize;

/// `permissions` for a workflow, job, or step.
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Permissions {
    Base(BasePermission),
    Explicit(ExplicitPermissions),
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

/// An "explicit" mapping of individual permissions.
///
/// Permissions that are not explicitly specified default to [`Permission::None`].
#[derive(Deserialize, Default, Debug, PartialEq)]
#[serde(rename_all = "kebab-case", default)]
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
#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
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

/// A "literal or expr" type, for places in GitHub Actions where a
/// key can either have a literal value (array, object, etc.) or an
/// expression string.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoE<T> {
    Literal(T),
    Expr(String),
}

impl<T> Default for LoE<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::Literal(T::default())
    }
}

/// A `bool` literal or an actions expression.
pub type BoE = LoE<bool>;

/// A "scalar or vector" type, for places in GitHub Actions where a
/// key can have either a scalar value or an array of values.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SoV<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Default for SoV<T> {
    fn default() -> Self {
        SoV::Many(Default::default())
    }
}

impl<T> SoV<T> {
    pub fn one(v: T) -> Self {
        SoV::One(v)
    }

    pub fn many(vs: Vec<T>) -> Self {
        SoV::Many(vs)
    }
}

impl<'a, T> IntoIterator for &'a SoV<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let slice = match self {
            SoV::One(v) => std::slice::from_ref(v),
            SoV::Many(vs) => vs.as_slice(),
        };

        slice.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{BasePermission, ExplicitPermissions};

    use super::{Permissions, SoV};

    #[test]
    fn test_permissions() {
        assert_eq!(
            serde_yaml::from_str::<Permissions>("read-all").unwrap(),
            Permissions::Base(BasePermission::ReadAll)
        );

        let perm = "security-events: write";
        assert!(matches!(
            serde_yaml::from_str::<ExplicitPermissions>(perm),
            Ok(_)
        ));
    }

    #[test]
    fn test_sov_intoiterator() {
        let sov_one = SoV::one("test".to_string());
        assert_eq!(sov_one.into_iter().collect::<Vec<_>>(), vec!["test"]);

        let sov_many = SoV::many(vec!["test-1".to_string(), "test-2".to_string()]);
        assert!(matches!(sov_many, SoV::Many(_)));
        assert_eq!(
            sov_many.into_iter().collect::<Vec<_>>(),
            vec!["test-1", "test-2"]
        );
    }
}
