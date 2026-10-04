//! Reaction sentiment: positive, negative, or undefined.
//!
//! The three-way split is inspired by ChatKeeper's public reaction
//! classification, but the tables below are our own and may differ from
//! theirs. Unlisted emoji (including custom reactions) are [`Unknown`],
//! never silently forced into a sentiment.

use serde::{Deserialize, Serialize};

/// Sentiment of one reaction emoji.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ReactionSentiment {
    Positive,
    Negative,
    Undefined,
    Unknown,
}

/// Aggregate counts for a batch of reactions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SentimentCounts {
    pub positive: u64,
    pub negative: u64,
    pub undefined: u64,
    pub unknown: u64,
}

impl SentimentCounts {
    pub fn add(&mut self, emoji: &str) {
        match classify(emoji) {
            ReactionSentiment::Positive => self.positive += 1,
            ReactionSentiment::Negative => self.negative += 1,
            ReactionSentiment::Undefined => self.undefined += 1,
            ReactionSentiment::Unknown => self.unknown += 1,
        }
    }

    pub fn total(&self) -> u64 {
        self.positive + self.negative + self.undefined + self.unknown
    }

    /// Share of positive reactions among decisive (positive + negative) ones.
    /// `None` when there are no decisive reactions.
    pub fn positivity(&self) -> Option<f64> {
        let decisive = self.positive + self.negative;
        if decisive == 0 {
            return None;
        }
        Some(self.positive as f64 / decisive as f64)
    }
}

/// Strip the emoji variation selector so `❤️` and `❤` classify identically.
fn skeleton(emoji: &str) -> String {
    emoji.chars().filter(|&ch| ch != '\u{fe0f}').collect()
}

/// Classify one reaction emoji. Empty strings (custom reactions without a
/// fallback glyph) are [`Unknown`].
pub fn classify(emoji: &str) -> ReactionSentiment {
    let base = skeleton(emoji);
    if base.is_empty() {
        return ReactionSentiment::Unknown;
    }
    if POSITIVE.contains(&base.as_str()) {
        ReactionSentiment::Positive
    } else if NEGATIVE.contains(&base.as_str()) {
        ReactionSentiment::Negative
    } else if UNDEFINED.contains(&base.as_str()) {
        ReactionSentiment::Undefined
    } else {
        ReactionSentiment::Unknown
    }
}

const POSITIVE: &[&str] = &[
    "👍",
    "❤",
    "🤝",
    "🔥",
    "🥰",
    "👏",
    "😁",
    "🎉",
    "🤩",
    "🙏",
    "👌",
    "🕊",
    "😍",
    "❤‍🔥",
    "💯",
    "🤣",
    "🏆",
    "🍓",
    "💋",
    "😇",
    "🤗",
    "🆒",
    "💘",
    "🦄",
    "😘",
    "😎",
    "🎄",
];

const NEGATIVE: &[&str] = &[
    "👎", "😱", "🤬", "😢", "🤮", "💩", "🤡", "🥱", "💔", "🖕", "😭", "😨", "😡",
];

const UNDEFINED: &[&str] = &[
    "🤔",
    "🤯",
    "🥴",
    "🐳",
    "🌚",
    "🌭",
    "⚡",
    "🍌",
    "🤨",
    "😐",
    "🍾",
    "😈",
    "😴",
    "🤓",
    "👻",
    "👨‍💻",
    "👀",
    "🎃",
    "🙈",
    "✍",
    "🫡",
    "🎅",
    "☃",
    "💅",
    "🤪",
    "🗿",
    "🙉",
    "💊",
    "🙊",
    "👾",
    "🤷‍♂",
    "🤷",
    "🤷‍♀",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_top_emoji_classify() {
        for emoji in [
            "👍", "😁", "💯", "🤣", "🔥", "👏", "🥰", "🤝", "🤩", "🎉", "👌",
        ] {
            assert_eq!(classify(emoji), ReactionSentiment::Positive, "{emoji}");
        }
        for emoji in ["🤡", "😢", "👎", "💩", "😱"] {
            assert_eq!(classify(emoji), ReactionSentiment::Negative, "{emoji}");
        }
        for emoji in ["🤔", "🤯", "⚡"] {
            assert_eq!(classify(emoji), ReactionSentiment::Undefined, "{emoji}");
        }
    }

    #[test]
    fn variation_selector_and_custom_emoji() {
        assert_eq!(classify("❤️"), ReactionSentiment::Positive);
        assert_eq!(classify("❤"), ReactionSentiment::Positive);
        assert_eq!(classify(""), ReactionSentiment::Unknown);
        assert_eq!(classify("🦄"), ReactionSentiment::Positive);
    }

    #[test]
    fn counts_and_positivity() {
        let mut counts = SentimentCounts::default();
        for emoji in ["👍", "👍", "👎", "🤔", ""] {
            counts.add(emoji);
        }
        assert_eq!(counts.total(), 5);
        assert_eq!(counts.positivity(), Some(2.0 / 3.0));
        assert_eq!(SentimentCounts::default().positivity(), None);
    }
}
