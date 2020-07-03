use anyhow::Result;
use app_dirs::{app_root, AppDataType, AppInfo};
use csv;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::num::ParseIntError;
use std::{env, fmt, process, str};
use structopt::StructOpt;

const TD_HOME: AppInfo = AppInfo {
    name: "td",
    author: "wrrb",
};

#[derive(Debug, StructOpt)]
#[structopt(name = "td", about = "A todo list")]
enum Cli {
    #[structopt(name = "+", about = "Add a new todo")]
    Add { message: String },
    #[structopt(name = "-", about = "Remove an old todo")]
    Rm { index: i8 },
}

fn main() {
    env_logger::init();

    debug!("start main");

    // create the td database file if it doesn't exist
    let log = Log::read_or_create().expect("Unable to read or create log file");

    // if no args, print and bail
    let has_args = env::args().nth(1).is_some();
    if !has_args {
        log.print();
        debug!("no args: bail");
        process::exit(0);
    }

    // handle args
    let args: Cli = Cli::from_args();
    debug!("current args: {:?}", args);
    match args {
        Cli::Add { message: m } => log.save(LogEntry::from_message(m)),
        Cli::Rm { index: i } => log.delete(LogEntry::from_index(i)),
    }

    debug!("end main");
}
// ----------------- Log ------------------- //

#[derive(Debug)]
struct Log {
    log_file: File,
    log_entries: Vec<LogEntry>,
}

impl Log {
    fn print(&self) {
        debug!("reading log");
        for entry in &self.log_entries {
            println!("{:?}", entry)
        }
    }

    fn read_or_create() -> Result<Log> {
        let log_file = Log::create_if_not_exists()?;
        let mut log = csv::Reader::from_reader(&log_file);

        let mut entries: Vec<LogEntry> = vec![];

        for result in log.deserialize() {
            let entry: LogEntry = result?;
            entries.push(entry);
        }

        Ok(Log {
            log_file,
            log_entries: entries,
        })
    }

    fn create_if_not_exists() -> Result<File> {
        let log_filename = "log";

        debug!("ensuring app_root");
        let _abspath_dir = app_root(AppDataType::UserConfig, &TD_HOME)?;

        debug!(
            "ensuring log file in dir: {}",
            _abspath_dir.to_str().unwrap_or("[error]")
        );
        let _abspath_log = _abspath_dir.join(log_filename.to_string());
        let file = fs::OpenOptions::new()
            .append(true)
            .read(true)
            .create(true)
            .open(&_abspath_log)?;

        Ok(file)
    }

    fn save(&self, entry: LogEntry) {
        let mut no_entries = false;
        if self.log_entries.len() == 0 as usize{
            no_entries = true;
        }
        debug!("saving LogEntry: {:?}", entry);
        let mut writer = csv::WriterBuilder::new()
            .has_headers(no_entries)
            .from_writer(&self.log_file);
        let result = writer.serialize(&entry);
        debug!("{:?} saved: {:?}", result, entry)
    }

    fn delete(&self, entry: LogEntry) {
        debug!("deleting LogEntry: {:?}", entry)
    }
}

// --------------- LogEntry ----------------- //

#[derive(Deserialize, Serialize)]
struct LogEntry {
    index: i8,
    message: String,
}

impl fmt::Debug for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LogEntry")
            .field("index", &self.index)
            .field("message", &self.message)
            .finish()
    }
}

impl str::FromStr for LogEntry {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LogEntry::from_message(s.to_string()))
    }
}

impl LogEntry {
    fn from_message(s: String) -> LogEntry {
        LogEntry {
            index: -1,
            message: s,
        }
    }

    fn from_index(index: i8) -> LogEntry {
        LogEntry {
            index,
            message: "dummy".to_string(),
        }
    }
}
