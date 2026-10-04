//! Shared formatting helpers with no Telegram dependency.
//!
//! The crate never escapes or links raw user input itself: callers pass
//! already-escaped names, which keeps HTML-safety decisions in the
//! application (see the `Html` builder pattern). `chat_id` values use the
//! `t.me/c/` short form, so callers must pass the bare supergroup id
//! (without the `-100` prefix).

/// Public link to a message in a supergroup/channel discussion.
pub fn message_link(bare_chat_id: i64, message_id: i64) -> String {
    format!("https://t.me/c/{bare_chat_id}/{message_id}")
}

/// Mention link for a user id with an already-escaped display name.
pub fn user_link(user_id: i64, escaped_name: &str) -> String {
    format!("<a href=\"tg://user?id={user_id}\">{escaped_name}</a>")
}

/// First `limit` characters plus an ellipsis marker when truncated.
/// `limit == 0` yields just the marker.
pub fn first_text_chars(text: &str, limit: usize) -> String {
    let trimmed = text.trim();
    if trimmed.chars().count() <= limit {
        return trimmed.to_string();
    }
    if limit == 0 {
        return "…".to_string();
    }
    format!("{}…", trimmed.chars().take(limit).collect::<String>())
}

/// Display name from optional parts. `None` when everything is blank so the
/// application can substitute its own localized fallback.
pub fn display_name(first_name: &str, last_name: &str) -> Option<String> {
    let name = format!("{} {}", first_name.trim(), last_name.trim())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_use_documented_schemes() {
        assert_eq!(message_link(1932061163, 42), "https://t.me/c/1932061163/42");
        assert_eq!(
            user_link(123, "Alice"),
            "<a href=\"tg://user?id=123\">Alice</a>"
        );
    }

    #[test]
    fn truncation_and_display_names() {
        assert_eq!(first_text_chars("abcdef", 3), "abc…");
        assert_eq!(first_text_chars("abc", 3), "abc");
        assert_eq!(display_name("  ", ""), None);
        assert_eq!(display_name("Ann", "Lee").as_deref(), Some("Ann Lee"));
    }
}
