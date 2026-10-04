//! Versioned linear reputation heads over small stable feature vectors.
//!
//! Like the text and embedding spam heads, this is a frozen-feature linear
//! layer: the consumer computes the documented features, the head returns a
//! supporting probability. Feature order is fixed by the artifact; unknown
//! feature names are rejected at load time.

use serde::Deserialize;

/// Stable reputation feature names. Append-only: never rename.
///
/// All features are point-in-time numerics: the 4PL id prior, the audit risk
/// score, log activity counts, the received-reaction sentiment shares, the
/// template-duplication counters, and log account age. See `tools/train_reputation.py`.
pub const FEATURES_V1: &[&str] = &[
    "id_prior",
    "audit_risk",
    "message_count_log",
    "active_days_log",
    "link_ratio",
    "dup_reuse_log",
    "positivity",
    "negativity_received",
    "account_age_days_log",
];

/// Versioned reputation head: `sigmoid(dot(weights, x) + intercept)`.
#[derive(Debug, Clone)]
pub struct ReputationModel {
    pub version: String,
    pub features: Vec<String>,
    pub weights: Vec<f64>,
    pub intercept: f64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ReputationExport {
    version: String,
    features: Vec<String>,
    weights: Vec<f64>,
    intercept: f64,
}

impl ReputationModel {
    pub fn load(json: &str) -> anyhow::Result<Self> {
        let export: ReputationExport = serde_json::from_str(json)?;
        if export.version.trim().is_empty() {
            anyhow::bail!("reputation model requires a version");
        }
        if export.features.is_empty()
            || export.features.len() != export.weights.len()
            || export.features.iter().any(|name| name.trim().is_empty())
        {
            anyhow::bail!("reputation features/weights mismatch or empty");
        }
        if !export.intercept.is_finite() || !export.weights.iter().all(|weight| weight.is_finite())
        {
            anyhow::bail!("reputation weights must be finite");
        }
        Ok(Self {
            version: export.version,
            features: export.features,
            weights: export.weights,
            intercept: export.intercept,
        })
    }

    /// Score a feature vector in artifact order. `None` on length mismatch
    /// or non-finite inputs: unusable features are no evidence.
    pub fn score(&self, values: &[f64]) -> Option<f64> {
        if values.len() != self.weights.len() || values.iter().any(|v| !v.is_finite()) {
            return None;
        }
        let dot: f64 = self
            .weights
            .iter()
            .zip(values)
            .map(|(weight, value)| weight * value)
            .sum();
        Some(1.0 / (1.0 + (-(dot + self.intercept)).exp()))
    }

    /// Look up one feature value by name from a caller-supplied map.
    pub fn feature_index(&self, name: &str) -> Option<usize> {
        self.features.iter().position(|feature| feature == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model() -> ReputationModel {
        ReputationModel::load(
            r#"{"version":"test-v1","features":["a","b"],"weights":[1.0,-1.0],"intercept":0.0}"#,
        )
        .unwrap()
    }

    #[test]
    fn scores_and_rejects_bad_inputs() {
        let model = model();
        let even = model.score(&[1.0, 1.0]).unwrap();
        assert!((even - 0.5).abs() < 1e-12);
        assert!(model.score(&[1.0, f64::NAN]).is_none());
        assert!(model.score(&[1.0]).is_none());
        assert_eq!(model.feature_index("b"), Some(1));
        assert_eq!(model.feature_index("zzz"), None);
    }

    #[test]
    fn loader_rejects_mismatch_and_nonfinite() {
        assert!(
            ReputationModel::load(
                r#"{"version":"v","features":["a"],"weights":[],"intercept":0.0}"#
            )
            .is_err()
        );
        assert!(
            ReputationModel::load(
                r#"{"version":"","features":["a"],"weights":[1.0],"intercept":0.0}"#
            )
            .is_err()
        );
        assert!(
            ReputationModel::load(
                r#"{"version":"v","features":["a"],"weights":[1.0],"intercept":0.0,"extra":1}"#
            )
            .is_err()
        );
    }
}
