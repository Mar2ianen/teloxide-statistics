# teloxide-statistics

An independent Rust statistics core for Telegram and community bots.
Pure computation only: reaction sentiment, engagement math, retention
cohorts, formatting helpers, and versioned reputation heads. SQL queries,
Telegram API calls, persistence, and message delivery belong to the
consuming application.

## Features

| Feature | Functionality |
|---|---|
| No defaults | Nothing (explicit opt-in) |
| `sentiment` | Reaction sentiment (positive/negative/undefined/unknown) with aggregate counts |
| `engagement` | Engagement/answered rates, ChatKeeper-style user segments, retention cohort tables |
| `render` | `t.me/c` message links, `tg://user` mention links, truncation, display names |
| `reputation` | Versioned linear reputation heads over small stable feature vectors |

Default: `render`. Full stack:

```toml
teloxide-statistics = { git = "https://github.com/Mar2ianen/teloxide-statistics", tag = "v0.1.0", features = ["sentiment", "engagement", "reputation"] }
```

Reaction sentiment tables are inspired by ChatKeeper's public
classification but are our own and may differ. Unlisted emoji — including
custom reactions — are `Unknown`, never forced into a sentiment.

Reputation heads are supporting evidence, like the antispam classifier
scores: `sigmoid(dot + intercept)` over caller-computed features in
artifact order. Train heads with `tools/train_reputation.py` on
adjudicated labels; publish weights and methodology, never datasets.

## Development

```sh
cargo fmt --all -- --check
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
python3 -m unittest discover -s tools -p 'test_*.py'
```

CI also tests each feature independently. Licensed under MIT.
