//! Independent chat statistics core for Telegram and community bots.
//!
//! Pure computation only: reaction sentiment, engagement math, retention
//! cohorts, formatting helpers, and versioned reputation heads. SQL queries,
//! Telegram API calls, persistence, and rendering into bot messages belong to
//! the consuming application.

#[cfg(feature = "engagement")]
pub mod engagement;
#[cfg(feature = "render")]
pub mod format;
#[cfg(feature = "reputation")]
pub mod reputation;
#[cfg(feature = "sentiment")]
pub mod sentiment;
