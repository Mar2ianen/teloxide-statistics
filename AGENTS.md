# teloxide-statistics

Independent Rust library. Do not introduce dependencies on a specific bot,
SQL schema, owner, Telegram chat, or LLM provider. Querying, snapshot
loading, persistence, and delivery belong to the consumer.

- Documentation, comments, errors, and commit messages must be in English.
  Non-English strings are allowed for text-processing rules and test inputs.
- Feature matrix: no-default, sentiment, engagement, render, reputation, default, all.
- Before committing: cargo fmt --check, cargo test --all-targets --all-features,
  cargo clippy --all-targets --all-features -- -D warnings, and the feature matrix.
- Never commit private corpora, moderation exports, credentials, or DSNs.
- Publish methods, aggregate metrics, and weights, not datasets or identifying
  examples. Data access requests require separate provenance/privacy review.
- Scores are supporting evidence, never standalone permission to punish.
  Preprocessing, artifacts, and calibration must have explicit versions.
- New tests use synthetic inputs and must not reference neighboring repositories.
