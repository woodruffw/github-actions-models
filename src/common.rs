use std::collections::HashMap;

use serde::Deserialize;

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

pub type BoE = LoE<bool>;

/// A "scalar or vector" type, for places in GitHub Actions where a
/// key can have either a scalar value or an array of values.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SoV<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> From<Vec<T>> for SoV<T> {
    fn from(value: Vec<T>) -> Self {
        Self::Many(value)
    }
}

impl<T> From<T> for SoV<T> {
    fn from(value: T) -> Self {
        Self::One(value)
    }
}
