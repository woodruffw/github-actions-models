//! Shared models and utilities.

use std::{borrow::Cow, collections::HashMap, fmt::Display};

use serde::{Deserialize, Deserializer, Serialize};

mod expr;

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

/// Represents a GitHub Actions expression.
///
/// This type performs no syntax checking on the underlying expression,
/// meaning that it might be invalid. The underlying expression may also
/// be "curly" or "bare" depending on its origin; use an appropriate
/// method like [`Expression::as_curly`] to access a specific form.
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct Expression(String);

impl Expression {
    /// Returns the underlying expression with any whitespace trimmed.
    /// This is unlikely to be necessary in practice, but can happen
    /// if a user encapsulates the expression in a YAML string with
    /// additional whitespace.
    fn trimmed(&self) -> &str {
        self.0.trim()
    }

    /// Returns whether the underlying inner expression is "curly", i.e.
    /// includes the `${{ ... }}` expression delimiters.
    fn is_curly(&self) -> bool {
        self.trimmed().starts_with("${{") && self.trimmed().ends_with("}}")
    }

    /// Construct an `Expression` from the given value if and only if
    /// the value is already a "curly" expression.
    pub fn from_curly(value: String) -> Option<Self> {
        let expr = Self(value);
        if !expr.is_curly() {
            return None;
        }

        Some(expr)
    }

    /// Construct an `Expression` from the given value if and only if
    /// the value is already a "bare" expression.
    pub fn from_bare(value: String) -> Option<Self> {
        let expr = Self(value);
        if expr.is_curly() {
            return None;
        }

        Some(expr)
    }

    /// Returns the "curly" form of this expression, i.e. `${{ expr }}`.
    pub fn as_curly(&self) -> Cow<'_, str> {
        if self.is_curly() {
            Cow::Borrowed(self.trimmed())
        } else {
            Cow::Owned(format!("${{{{ {expr} }}}}", expr = self.trimmed()))
        }
    }

    /// Returns the "bare" form of this expression, i.e. `expr` if
    /// the underlying expression is `${{ expr }}`.
    pub fn as_bare(&self) -> &str {
        if self.is_curly() {
            self.trimmed()
                .strip_prefix("${{")
                .unwrap()
                .strip_suffix("}}")
                .unwrap()
                .trim()
        } else {
            self.trimmed()
        }
    }
}

/// A "literal or expr" type, for places in GitHub Actions where a
/// key can either have a literal value (array, object, etc.) or an
/// expression string.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoE<T> {
    Literal(T),
    Expr(Expression),
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

pub(crate) fn bool_is_string<'de, D>(de: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoS::deserialize(de).map(Into::into)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::common::{BasePermission, Permission};

    use super::{Expression, Permissions};

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
    fn test_expression() {
        let expr = Expression("${{ foo }}".to_string());
        assert_eq!(expr.as_curly(), "${{ foo }}");
        assert_eq!(expr.as_bare(), "foo");

        let expr = Expression("foo".to_string());
        assert_eq!(expr.as_curly(), "${{ foo }}");
        assert_eq!(expr.as_bare(), "foo");

        let expr = Expression(" \t ${{ foo  }} \t\n".to_string());
        // NOTE: whitespace within the curly is preserved. Worth changing?
        assert_eq!(expr.as_curly(), "${{ foo  }}");
        assert_eq!(expr.as_bare(), "foo");

        let expr = Expression("  foo \t\n".to_string());
        assert_eq!(expr.as_curly(), "${{ foo }}");
        assert_eq!(expr.as_bare(), "foo");
    }

    #[test]
    fn test_expression_from_curly() {
        assert!(Expression::from_curly("${{ foo }}".into()).is_some());
        assert!(Expression::from_curly("foo".into()).is_none());
    }

    #[test]
    fn test_expression_from_bare() {
        assert!(Expression::from_bare("${{ foo }}".into()).is_none());
        assert!(Expression::from_bare("foo".into()).is_some());
    }
}
