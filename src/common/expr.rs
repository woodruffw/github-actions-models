//! GitHub Actions expression parsing and handling.

use serde::Deserialize;

/// An explicit GitHub Actions expression, fenced by `${{ <expr> }}`.
pub struct ExplicitExpr(String);

impl AsRef<str> for ExplicitExpr {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<'de> Deserialize<'de> for ExplicitExpr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        let raw = raw.trim();

        let Some(inner) = raw.strip_prefix("${{").and_then(|r| r.strip_suffix("}}")) else {
            return Err(serde::de::Error::custom(
                "invalid expression: expected '${{' and '}}' delimiters",
            ));
        };

        Ok(Self(inner.trim().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::ExplicitExpr;

    #[test]
    fn test_expr_invalid() {
        let cases = &[
            "not an expression",
            "${{ missing end ",
            "missing beginning }}",
        ];

        for case in cases {
            let case = format!("\"{case}\"");
            assert!(serde_yaml::from_str::<ExplicitExpr>(&case).is_err());
        }
    }

    #[test]
    fn test_expr() {
        let expr = "\"  ${{ foo }} \\t \"";
        let expr: ExplicitExpr = serde_yaml::from_str(&expr).unwrap();
        assert_eq!(expr.as_ref(), "foo");
    }
}
