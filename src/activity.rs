//! Shared per-user activity counters for chat statistics consumers.
//!
//! This is a data model only. Consumers own the query, population filters,
//! persistence, and presentation. Every frontend for one user should render
//! the same value instead of calculating its own variant of a counter.

use serde::{Deserialize, Serialize};

/// Canonical activity totals for one user in one chat and one reporting scope.
///
/// `reactions_received` counts the current reaction total on the user's messages.
/// Historical reaction actions have a different meaning and are consumer-specific.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserActivity {
    pub messages: i64,
    pub replies: i64,
    pub links: i64,
    pub media: i64,
    pub voices: i64,
    pub reactions_received: i64,
}
