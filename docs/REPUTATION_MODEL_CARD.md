# Reputation v1 — training and operating notes

Model: `rep-v1-2026-10-04`.
Features: nine point-in-time numerics (`FEATURES_V1`), no texts or user IDs
in the training rows. Head: `LogisticRegression(C=0.5, balanced)`.
Release asset: `rep_v1_2026-10-04.json`.
SHA256: `0263d8a593d377b9e02c6e3d59ba5c0cf0042c81b9a391e582701755e8cb7b43`.

## Labels

Reviewer decisions from `spam_label_events` joined to the latest audit
snapshot at or before label time (3,384 users after dropping 1 conflicting
and 164 without a prior snapshot):

- spam: 173 users — 86 `owner_manual`, ~87 `legacy_backfill` (medium trust).
- not_spam: 3,211 users — 19 `owner_manual`, the rest `system`
  `archive_assumed_normal` (weak: assumed normal, may hide undetected spam).

Weak negatives dominate, so the head mostly learns "reviewed risk plus
normal-activity corrections". It must stay supporting-only and must never
gate enforcement: high `audit_risk` already selected most positives for
review, so the 6.255 `audit_risk` weight partly restates the review policy.

## Cross-validation (stratified 5-fold, out-of-fold)

ROC-AUC 0.983, PR-AUC 0.88. `audit_risk` alone already reaches ROC 0.978;
the remaining features add texture (fresh-account, activity, sentiment),
not a new verdict.

## Notable finding: the fresh-id prior points the wrong way here

Mean 4PL id prior: 0.468 (spam) vs 0.984 (ham). Recent IDs belong mostly to
legitimate newcomers in this growing chat; caught spammers skew older
(bought/takeover accounts). The fitted `id_prior` weight is −4.139. The
baseline monotonic "fresh id is risky" prior should be re-examined for this
community separately — this head compensates locally, it does not fix the
baseline.

## Coefficients

`id_prior` −4.139, `audit_risk` +6.255, `message_count_log` −1.056,
`active_days_log` −1.088, `link_ratio` −0.512, `dup_reuse_log` −0.149,
`positivity` −2.661, `negativity_received` −2.050,
`account_age_days_log` −0.739, intercept +4.397. The negative
`negativity_received` weight is likely small-sample noise (spammers get few
reactions at all); treat it as regularized-down, not as signal semantics.

## Intended use

Review routing and card context only. Refit when the label mix changes;
re-examine the baseline id prior independently. No private rows, texts, or
user IDs are published — weights and methodology only.
