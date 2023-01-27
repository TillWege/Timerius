use std::{num::ParseIntError, thread, path::PathBuf};
use clap::{Parser, command, Subcommand};
use ini::{Ini, Error};

use notify_rust::Notification;

#[derive(Debug)]
struct Timer {
    name: String,
    notification: String,
    start: std::time::Instant,
    interval: std::time::Duration, // in seconds
    repeating: bool,
    ended: bool,
}

/// A Program to set a multitude of asynchronous timers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    // Sets config file on which to operate
    #[arg(value_name = "CONFIG_FILE", default_value = "config.ini")]
    config: PathBuf,
    
    // The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,    
}

#[derive(Subcommand, Debug)]
enum Commands {
    Start,
    List,
    Add {
        // Name of the new Timer
        #[arg(short, long)]
        name: String,
        
        // Notification to show when the timer is up
        #[arg(short, long)]
        notification: String,
        
        // Interval in seconds after which the notification should be shown
        #[arg(short, long)]
        interval: u64,

        // Should the timer repeat after it is up
        #[arg(short, long)]
        repeating: bool,
    },
    Remove,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let conf = Ini::load_from_file(args.config)?;

    
    match args.command {
        Some(Commands::Start) => {
                let timers = load_timers_from_file(conf);
                start_timers(timers);
        },        
        Some(Commands::List) => todo!(),
        Some(Commands::Add { name, notification, interval, repeating }) => todo!(),
        Some(Commands::Remove) => todo!(),
        None => return Ok(()),
    }
    
    Ok(())
}

fn load_timers_from_file(ini: Ini) -> Vec<Timer> {
    let mut timers: Vec<Timer> = Vec::new();
    for section in ini.sections() {
        if let Some(section) = section {
            let name = ini.get_from(Some(section), "name").unwrap_or(section).to_string();
            let notification = ini.get_from(Some(section), "notification").unwrap_or(section).to_string();
            let interval = ini.get_from(Some(section), "interval").unwrap_or("0");
            let interval = interval.parse::<u64>().unwrap();
            let repeating = ini.get_from(Some(section), "repeating").unwrap();
            let repeating = repeating.parse::<bool>().unwrap();
            timers.push( Timer {
                name: name.to_string(),
                notification: notification.to_string(),
                start: std::time::Instant::now(),
                interval: std::time::Duration::from_secs(interval),
                repeating,
                ended: false,
            });
        }
    }

    timers
}

fn start_timers(mut timers: Vec<Timer>) {
    while (timers.iter().map(|x| x.ended).collect::<Vec<bool>>()).contains(&false) {
        for timer in &mut timers {
            if timer.start.elapsed() >= timer.interval {
                Notification::new()
                    .summary(&timer.name)
                    .body(&timer.notification)
                    .show()
                    .unwrap();
                if timer.repeating {
                    timer.start = std::time::Instant::now();
                } else {
                    timer.ended = true;
                }
            }
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}