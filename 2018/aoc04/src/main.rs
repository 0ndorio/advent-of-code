use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    str::FromStr,
};

use chrono::{NaiveDateTime, Timelike};
use lazy_static::lazy_static;
use regex::Regex;

type Result<ContentT> = std::result::Result<ContentT, Box<dyn Error>>;
type Overview = HashMap<u32, (u32, [u32; 60])>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = parse_log_entries(&input)?;
    let overview = generate_log_overview(&entries);

    let first_metric = calc_sleepiest_guard_metric(&overview);
    println!("Metric for the first strategy: {}", first_metric);
    Ok(())
}

fn parse_log_entries(input: &str) -> Result<Vec<WatchEntry>> {
    let mut entries: Vec<WatchEntry> = input
        .split('\n')
        .map(WatchEntry::from_str)
        .flatten()
        .collect();

    entries.sort();
    Ok(entries)
}

fn generate_log_overview(entries: &[WatchEntry]) -> Overview {
    let mut overview = HashMap::<u32, (u32, [u32; 60])>::new();

    let mut guard_log = &mut (0, [0; 60]);
    let mut start_sleep = 0;
    for entry in entries {
        match entry.action {
            Action::ShiftBegin(id) => guard_log = overview.entry(id).or_insert((0, [0; 60])),
            Action::FallsAsleep => start_sleep = entry.time.minute(),
            Action::Awakes => {
                let end_sleep = entry.time.minute();
                guard_log.0 += end_sleep - start_sleep;

                for minute in start_sleep..end_sleep {
                    let minute = minute as usize;
                    guard_log.1[minute] += 1;
                }
            }
        }
    }

    overview
}

fn calc_sleepiest_guard_metric(overview: &Overview) -> usize {
    let (id, (_, log)) = overview
        .iter()
        .max_by(|(_, (total_minutes_lhs, _)), (_, (total_minutes_rhs, _))| {
            total_minutes_lhs.cmp(total_minutes_rhs)
        })
        .expect("Overview should contain at least one guard.");

    let (minute, _) = log
        .iter()
        .enumerate()
        .max_by(|(_, sleeping_periods_lhs), (_, sleeping_periods_rhs)| {
            sleeping_periods_lhs.cmp(sleeping_periods_rhs)
        })
        .expect("Every guard entry should contain 60 values");

    (*id as usize) * minute
}

// -----------------------------------------------------------------------------
// WatchEntry
// -----------------------------------------------------------------------------

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct WatchEntry {
    time: NaiveDateTime,
    action: Action,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Action {
    ShiftBegin(u32),
    FallsAsleep,
    Awakes,
}

impl FromStr for WatchEntry {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        lazy_static! {
            static ref LOG_ENTRY_RE: Regex = Regex::new(r"^\[(?P<time>.+)\] (?P<content>.*)",)
                .expect("Predefined log entry regex failed to compile.");
        }

        let capture = LOG_ENTRY_RE
            .captures(input)
            .ok_or_else(|| format!("Couldn't parse log entry line: {}", input))?;

        let time = NaiveDateTime::parse_from_str(&capture["time"], "%Y-%m-%d %H:%M")?;
        let action = capture["content"].parse()?;

        Ok(WatchEntry { time, action })
    }
}

impl FromStr for Action {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        if input.contains("falls asleep") {
            return Ok(Action::FallsAsleep);
        }

        if input.contains("wakes up") {
            return Ok(Action::Awakes);
        }

        if input.contains("begins shift") {
            lazy_static! {
                static ref GUARD_ID_RE: Regex = Regex::new(r"#(?P<id>\d+)",)
                    .expect("Predefined guard id regex failed to compile.");
            }

            let capture = GUARD_ID_RE
                .captures(input)
                .ok_or_else(|| format!("Can't find any guard id in: {}", input))?;

            let id = capture["id"].parse()?;
            return Ok(Action::ShiftBegin(id));
        }

        Err(Box::from(format!(
            "Can't find any guard action in: {}",
            input
        )))
    }
}
