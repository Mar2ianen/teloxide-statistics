//! Engagement math: rates, segments, and retention cohorts.
//!
//! All functions are pure over caller-supplied counts. Day indices are
//! caller-defined (e.g. days since epoch in the render timezone).

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Share of members who wrote at least once. `None` on zero members.
pub fn engagement_rate(active_users: u64, total_members: u64) -> Option<f64> {
    if total_members == 0 {
        return None;
    }
    Some(active_users as f64 / total_members as f64)
}

/// Share of messages that received at least one reply. `None` on zero input.
pub fn answered_share(replied_messages: u64, total_messages: u64) -> Option<f64> {
    if total_messages == 0 {
        return None;
    }
    Some(replied_messages as f64 / total_messages as f64)
}

/// ChatKeeper-style audience segment for one user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserSegment {
    /// Present under 14 days with at least one message.
    New,
    /// Writing on more than 30 distinct days.
    Regular,
    /// Everyone else with at least one message.
    Active,
}

/// `days_present`: days since first seen; `active_days`: distinct days with
/// at least one message; `messages`: total messages. Users with zero messages
/// have no segment.
pub fn segment_user(days_present: u64, active_days: u64, messages: u64) -> Option<UserSegment> {
    if messages == 0 {
        return None;
    }
    if days_present < 14 {
        Some(UserSegment::New)
    } else if active_days > 30 {
        Some(UserSegment::Regular)
    } else {
        Some(UserSegment::Active)
    }
}

/// Retention cohort table: for each cohort day (first active day), how many
/// users were active again `offset` days later. Offsets run `0..=max_offset`.
/// Input is `(user_id, day)` activity pairs; days need not be consecutive.
pub fn retention_table(activity: &[(i64, i64)], max_offset: i64) -> BTreeMap<i64, Vec<u64>> {
    let mut first_seen: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    let mut active: BTreeMap<i64, BTreeSet<i64>> = BTreeMap::new();
    for &(user, day) in activity {
        first_seen.entry(user).or_default().insert(day);
        active.entry(day).or_default().insert(user);
    }
    let mut cohorts: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    for (user, days) in &first_seen {
        let cohort = *days.iter().min().expect("nonempty day set");
        cohorts.entry(cohort).or_default().push(*user);
    }
    cohorts
        .into_iter()
        .map(|(cohort, users)| {
            let row = (0..=max_offset)
                .map(|offset| {
                    active
                        .get(&(cohort + offset))
                        .map(|day_users| {
                            users.iter().filter(|user| day_users.contains(user)).count() as u64
                        })
                        .unwrap_or(0)
                })
                .collect();
            (cohort, row)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rates_reject_empty_denominators() {
        assert_eq!(engagement_rate(15, 100), Some(0.15));
        assert_eq!(engagement_rate(0, 0), None);
        assert_eq!(answered_share(8, 10), Some(0.8));
        assert_eq!(answered_share(0, 0), None);
    }

    #[test]
    fn segments_follow_new_regular_active_rules() {
        assert_eq!(segment_user(5, 3, 10), Some(UserSegment::New));
        assert_eq!(segment_user(40, 35, 100), Some(UserSegment::Regular));
        assert_eq!(segment_user(40, 10, 100), Some(UserSegment::Active));
        assert_eq!(segment_user(5, 0, 0), None);
    }

    #[test]
    fn retention_counts_returning_users() {
        // User 1: days 10, 12. User 2: day 10 only. User 3: days 12, 13.
        let activity = [(1, 10), (1, 12), (2, 10), (3, 12), (3, 13)];
        let table = retention_table(&activity, 3);
        assert_eq!(table[&10], vec![2, 0, 1, 0]);
        assert_eq!(table[&12], vec![1, 1, 0, 0]);
    }
}
