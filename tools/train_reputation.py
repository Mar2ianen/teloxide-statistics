"""Fit a tiny logistic reputation head on adjudicated user labels.

Input is a local JSONL with numeric point-in-time features only (no texts,
no user IDs): {"label": 0|1, "features": {"id_prior": 0.5, ...}} using the
nine FEATURES_V2 names. Labels must come from reviewer decisions joined to
the latest audit snapshot at or before label time; conflicting users are
dropped upstream. See docs/REPUTATION_MODEL_CARD.md.

Outputs a versioned JSON head loadable by
`teloxide_statistics::reputation::ReputationModel`. Supporting evidence
only, never a punishment authorization.
"""

import argparse
import json
from pathlib import Path

import numpy as np
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import average_precision_score, roc_auc_score
from sklearn.model_selection import StratifiedKFold, cross_val_predict

FEATURES_V2 = [
    "id_prior",
    "audit_risk",
    "message_count_log",
    "active_days_log",
    "link_ratio",
    "dup_reuse_log",
    "positivity",
    "negativity_received",
    "account_age_days_log",
    "text_gemma_prob",
    "text_tfidf_prob",
    "has_text",
]


def load_rows(path):
    labels, matrix = [], []
    for line in Path(path).read_text().splitlines():
        if not line.strip():
            continue
        row = json.loads(line)
        assert row["label"] in (0, 1)
        vector = [row["features"][name] for name in FEATURES_V2]
        assert all(isinstance(v, (int, float)) and np.isfinite(v) for v in vector)
        labels.append(row["label"])
        matrix.append(vector)
    labels = np.array(labels)
    assert set(np.unique(labels)) == {0, 1}, "both classes required"
    return np.array(matrix, dtype=np.float64), labels


def evaluate(matrix, labels, seed=7):
    classifier = LogisticRegression(
        C=0.5, max_iter=2000, class_weight="balanced", random_state=42
    )
    probabilities = cross_val_predict(
        classifier,
        matrix,
        labels,
        cv=StratifiedKFold(5, shuffle=True, random_state=seed),
        method="predict_proba",
    )[:, 1]
    return {
        "roc_auc": float(roc_auc_score(labels, probabilities)),
        "pr_auc": float(average_precision_score(labels, probabilities)),
    }


def fit_head(matrix, labels):
    classifier = LogisticRegression(
        C=0.5, max_iter=2000, class_weight="balanced", random_state=42
    )
    classifier.fit(matrix, labels)
    return {
        "weights": [float(w) for w in classifier.coef_[0]],
        "intercept": float(classifier.intercept_[0]),
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rows", type=Path, help="private JSONL, see module docstring")
    parser.add_argument("--version", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    matrix, labels = load_rows(args.rows)
    print(json.dumps({"n": len(labels), "spam": int(labels.sum()), **evaluate(matrix, labels)}))
    head = fit_head(matrix, labels)
    args.output.write_text(
        json.dumps({"version": args.version, "features": FEATURES_V2, **head}) + "\n"
    )


if __name__ == "__main__":
    main()
