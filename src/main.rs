use std::fmt::Display;

use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Timestamp provided in one of the available `format`s.
    #[arg(short, long)]
    timestamp: String,

    /// The format that your `timestamp` is provided in.
    #[arg(long, default_value_t = Format::default())]
    format: Format,

    /// Include time information about the current timestamp.
    #[arg(short, long)]
    include_now: bool,
}

#[derive(ValueEnum, Default, Clone, Debug)]
#[allow(non_camel_case_types)]
enum Format {
    #[default]
    seconds,
    milliseconds,
    microseconds,
    nanoseconds,
}

impl Format {
    fn _symbol(&self) -> String {
        match self {
            Format::seconds => "s".to_string(),
            Format::milliseconds => "ms".to_string(),
            Format::microseconds => "μs".to_string(),
            Format::nanoseconds => "ns".to_string(),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Times {
    pub local_time: DateTime<Local>,
    pub utc: DateTime<Utc>,
    pub unix_time_s: i64,
    pub unix_time_ms: i64,
}

impl Times {
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self {
            local_time: DateTime::from(dt),
            utc: dt,
            unix_time_s: dt.timestamp(),
            unix_time_ms: dt.timestamp_millis(),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let in_time = cli.timestamp.parse::<i64>().expect("input is a valid i64");

    let provided =
        match cli.format {
            Format::seconds => {
                DateTime::<Utc>::from_timestamp(in_time, 0).expect("input should be a valid time")
            }
            Format::milliseconds => DateTime::<Utc>::from_timestamp_millis(in_time)
                .expect("input should be a valid time"),
            Format::microseconds => DateTime::<Utc>::from_timestamp_micros(in_time)
                .expect("input should be a valid time"),
            Format::nanoseconds => DateTime::<Utc>::from_timestamp_nanos(in_time),
        };

    let provided_times = Times::new(provided);

    if cli.include_now {
        let sys_utc = chrono::offset::Utc::now();

        let times = Times::new(sys_utc);

        println!("now    : {times:#?}");
    }

    println!("provied: {provided_times:#?}");

    Ok(())
}
