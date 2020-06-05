use env_logger;
use log::debug;
use std::num::ParseIntError;
use std::{fmt, str};
use structopt::StructOpt;
use app_dirs::{AppInfo, AppDataType, get_app_root, app_root};

const TD_HOME: AppInfo = AppInfo {name: "td", author: "wrrb"};

#[derive(Debug, StructOpt)]
#[structopt(name = "td", about = "A todo list")]
enum Cli {
    #[structopt(name = "+", about = "Add a new todo")]
    Add { message: String },
    #[structopt(name = "-", about = "Remove an old todo")]
    Rm { index: i8 },
}

fn main() {
    debug!("start main");

    env_logger::init();

    // create the td database file if it doesn't exist
    let log = Log::create_if_not_exists();

    let args: Cli = Cli::from_args();
    debug!("current args: {:?}", args);
    match args {
        Cli::Add { message: m } => log.save(LogEntry::from_message(m)),
        Cli::Rm { index: i } => log.delete(LogEntry::from_index(i)),
    }
}
// ----------------- Log ------------------- //

struct Log {
    dir: String,
    name: String,
    abspath: String,
    relpath: String,
    log_entries: Vec<LogEntry>,
}

impl Log {
    fn read(&self) {
        debug!("reading log");
        for entry in &self.log_entries {
            println!("{:?}", entry)
        }
    }

    fn create_if_not_exists() -> Log {
        debug!("checking if log exists");
        let abspath_dir = app_root(AppDataType::UserConfig, &TD_HOME);
        debug!("creating log");
        Log {
            dir: String::from(".td"),
            name: String::from("log"),
            abspath: String::from("/home/wrrb/.td/log"),
            relpath: String::from("$PWD/../.td/log"),
            log_entries: vec![LogEntry { index: 8, message: "from create".to_string()}],
        }
    }

    fn save(&self, entry: LogEntry) {
        debug!("saving LogEntry: {:?}", entry); 
    }

    fn delete(&self, entry: LogEntry) {
        debug!("deleting LogEntry: {:?}", entry)      
    }
}

// --------------- LogEntry ----------------- //

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
        Ok(LogEntry {
            index: -1,
            message: s.to_string(),
        })
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
            index: index,
            message: "dummy".to_string(),
        }
    }
}