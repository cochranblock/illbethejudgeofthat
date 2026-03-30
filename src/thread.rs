use std::collections::HashMap;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::ingest::T0;
use crate::analyze::f4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct T5 {
    pub thread_id: String,
    pub emails: Vec<usize>,
    pub participants: Vec<String>,
    pub subject: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub message_count: usize,
}

/// Reconstruct conversation threads from parsed emails.
/// 3-tier: X-GM-THRID → In-Reply-To/Message-ID chain → normalized subject within 7 days.
pub fn f2(emails: &[T0]) -> Vec<T5> {
    // Build lookup maps
    let mut msgid_to_idx: HashMap<String, usize> = HashMap::new();
    for (i, email) in emails.iter().enumerate() {
        if !email.message_id.is_empty() {
            msgid_to_idx.insert(email.message_id.clone(), i);
        }
    }

    // Union-find for grouping
    let n = emails.len();
    let mut parent: Vec<usize> = (0..n).collect();

    let find = |parent: &mut Vec<usize>, mut x: usize| -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        x
    };

    let union = |parent: &mut Vec<usize>, a: usize, b: usize| {
        let ra = {
            let mut x = a;
            while parent[x] != x { parent[x] = parent[parent[x]]; x = parent[x]; }
            x
        };
        let rb = {
            let mut x = b;
            while parent[x] != x { parent[x] = parent[parent[x]]; x = parent[x]; }
            x
        };
        if ra != rb {
            parent[ra] = rb;
        }
    };

    // Tier 1: Group by X-GM-THRID
    let mut thrid_groups: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, email) in emails.iter().enumerate() {
        if !email.thread_id.is_empty() {
            thrid_groups.entry(email.thread_id.clone()).or_default().push(i);
        }
    }
    for indices in thrid_groups.values() {
        for window in indices.windows(2) {
            union(&mut parent, window[0], window[1]);
        }
    }

    // Tier 2: Chain via In-Reply-To → Message-ID
    for (i, email) in emails.iter().enumerate() {
        if !email.in_reply_to.is_empty() {
            if let Some(&j) = msgid_to_idx.get(&email.in_reply_to) {
                union(&mut parent, i, j);
            }
        }
    }

    // Tier 3: Normalized subject match within 7-day window
    let mut subject_map: HashMap<String, Vec<(usize, Option<NaiveDate>)>> = HashMap::new();
    for (i, email) in emails.iter().enumerate() {
        let norm = normalize_subject(&email.subject);
        if norm.len() > 3 {
            let date = f4(&email.date);
            subject_map.entry(norm).or_default().push((i, date));
        }
    }
    for entries in subject_map.values() {
        for pair in entries.windows(2) {
            let (i, date_a) = &pair[0];
            let (j, date_b) = &pair[1];
            // Only merge if both already ungrouped (no THRID/reply link) and within 7 days
            let ri = find(&mut parent, *i);
            let rj = find(&mut parent, *j);
            if ri == rj {
                continue; // already grouped
            }
            if let (Some(da), Some(db)) = (date_a, date_b) {
                if (*da - *db).num_days().abs() <= 7 {
                    union(&mut parent, *i, *j);
                }
            }
        }
    }

    // Collect groups
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        groups.entry(root).or_default().push(i);
    }

    // Build Thread structs
    let mut threads: Vec<T5> = Vec::with_capacity(groups.len());
    for (_, mut indices) in groups {
        // Sort by date
        indices.sort_by(|a, b| {
            let da = f4(&emails[*a].date);
            let db = f4(&emails[*b].date);
            da.cmp(&db)
        });

        let first = &emails[indices[0]];
        let last = &emails[*indices.last().unwrap()];

        // Collect unique participants
        let mut participants: Vec<String> = Vec::new();
        for &idx in &indices {
            let from = extract_email_addr(&emails[idx].from);
            if !from.is_empty() && !participants.contains(&from) {
                participants.push(from);
            }
        }

        let thread_id = if !first.thread_id.is_empty() {
            first.thread_id.clone()
        } else {
            format!("synth-{}", indices[0])
        };

        threads.push(T5 {
            thread_id,
            message_count: indices.len(),
            subject: normalize_subject(&first.subject),
            start_date: f4(&first.date).map(|d| d.format("%Y-%m-%d").to_string()),
            end_date: f4(&last.date).map(|d| d.format("%Y-%m-%d").to_string()),
            participants,
            emails: indices,
        });
    }

    // Sort threads by start date
    threads.sort_by(|a, b| {
        let da = a.start_date.as_deref().unwrap_or("9999");
        let db = b.start_date.as_deref().unwrap_or("9999");
        da.cmp(db)
    });

    threads
}

/// Strip Re:, Fwd:, FW: prefixes and normalize whitespace
fn normalize_subject(subject: &str) -> String {
    let mut s = subject.to_string();
    loop {
        let trimmed = s.trim().to_string();
        let lower = trimmed.to_lowercase();
        if lower.starts_with("re:") {
            s = trimmed[3..].to_string();
        } else if lower.starts_with("fwd:") {
            s = trimmed[4..].to_string();
        } else if lower.starts_with("fw:") {
            s = trimmed[3..].to_string();
        } else {
            break;
        }
    }
    s.split_whitespace().collect::<Vec<&str>>().join(" ").to_lowercase()
}

/// Extract email address from "Name <addr>" or bare "addr" format
fn extract_email_addr(from: &str) -> String {
    if let Some(start) = from.find('<') {
        if let Some(end) = from.find('>') {
            return from[start + 1..end].to_lowercase();
        }
    }
    from.trim().to_lowercase()
}

/// Print thread summary
pub fn f3(threads: &[T5]) -> String {
    let mut out = String::new();
    out.push_str(&format!("Threads reconstructed: {}\n", threads.len()));

    let single = threads.iter().filter(|t| t.message_count == 1).count();
    let multi = threads.len() - single;
    out.push_str(&format!("  Multi-message threads: {}\n", multi));
    out.push_str(&format!("  Single-message threads: {}\n", single));

    // Top 10 longest threads
    let mut by_len: Vec<&T5> = threads.iter().collect();
    by_len.sort_by(|a, b| b.message_count.cmp(&a.message_count));
    out.push_str("\nTop 10 threads by length:\n");
    for t in by_len.iter().take(10) {
        out.push_str(&format!("  {:3} msgs | {} | {}\n",
            t.message_count,
            t.start_date.as_deref().unwrap_or("?"),
            truncate(&t.subject, 60),
        ));
    }

    out
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max { s.to_string() } else { format!("{}...", &s[..max]) }
}
