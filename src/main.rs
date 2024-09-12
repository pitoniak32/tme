use std::fmt::Display;

use anyhow::Result;
use chrono::{DateTime, Local, Utc};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Timestamp provided in one of the available `format`s.
    /// 
    /// you can provide the values in a csv list.
    /// They will all need to be in the same format
    ///
    /// `1725932348,1725932348,1725932348`
    ///
    /// If not provided, the current system time will be used.
    #[arg()]
    timestamp: Option<String>,

    /// The format that your `timestamp` is provided in.
    #[arg(short, long, default_value_t = Format::default())]
    format: Format,

    /// Include time information about the current system time.
    #[arg(short, long)]
    now: bool,
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
    fn symbol(&self) -> String {
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
    pub local: DateTime<Local>,
    pub utc: DateTime<Utc>,
    pub unix_s: i64,
    pub unix_ms: i64,
}

impl Times {
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self {
            local: DateTime::from(dt),
            utc: dt,
            unix_s: dt.timestamp(),
            unix_ms: dt.timestamp_millis(),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(timestamp) = &cli.timestamp {
        timestamp.split(",").filter_map(|val| {
            if val.trim().is_empty() { return None }
            if let Some(val_in) = val.parse::<i64>().ok() {
                return Some(val_in)
            } else {
                println!("Could not convert \"{val}\" into i64");
                return None
            }
        }).for_each(|ts| {
            let in_time = match cli.format {
                Format::seconds => {
                    DateTime::<Utc>::from_timestamp(ts, 0).expect("input should be a valid time")
                }
                Format::milliseconds => DateTime::<Utc>::from_timestamp_millis(ts)
                    .expect("input should be a valid time"),
                Format::microseconds => DateTime::<Utc>::from_timestamp_micros(ts)
                    .expect("input should be a valid time"),
                Format::nanoseconds => DateTime::<Utc>::from_timestamp_nanos(ts),
            };

            println!("({ts} {sym}): {time:#?}", sym = cli.format.symbol(), time = Times::new(in_time));
        })
    }

    if cli.now {
        println!("(now): {:#?}", Times::new(chrono::offset::Utc::now()));
    }

    Ok(())
}
