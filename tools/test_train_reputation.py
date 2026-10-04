import json
import unittest
from pathlib import Path

import numpy as np

import sys
sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    from train_reputation import FEATURES_V1, evaluate, fit_head, load_rows
    HAS_SKLEARN = True
except ImportError:
    HAS_SKLEARN = False


@unittest.skipUnless(HAS_SKLEARN, "scikit-learn/numpy required")
class ReputationToolTests(unittest.TestCase):
    def test_evaluate_and_fit_roundtrip(self):
        rng = np.random.default_rng(11)
        matrix = np.vstack([rng.normal(-1, 0.5, (30, 9)), rng.normal(1, 0.5, (30, 9))])
        labels = np.array([0] * 30 + [1] * 30)
        metrics = evaluate(matrix, labels)
        self.assertGreater(metrics["roc_auc"], 0.95)
        head = fit_head(matrix, labels)
        self.assertEqual(len(head["weights"]), 9)
        self.assertTrue(all(isinstance(w, float) for w in head["weights"]))

    def test_feature_order_is_stable(self):
        self.assertEqual(
            FEATURES_V1,
            ["id_prior", "audit_risk", "message_count_log", "active_days_log",
             "link_ratio", "dup_reuse_log", "positivity", "negativity_received",
             "account_age_days_log"],
        )


if __name__ == "__main__":
    unittest.main()
