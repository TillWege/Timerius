use clap::{command, Parser, Subcommand};
use ini::{Error, Ini};
use std::{path::PathBuf, thread};

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
    #[arg(value_name = "CONFIG_FILE", help="when this is not set, config.ini in the same folder as the executable will be used")]
    config: Option<PathBuf>,

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
        #[arg(short, long, default_value = "New Timer")]
        name: String,

        // Notification to show when the timer is up
        #[arg(short = 'd', long, default_value = "Timer is up")]
        notification: String,

        // Interval in seconds after which the notification should be shown
        #[arg(short, long, default_value = "60")]
        interval: u64,

        // Should the timer repeat after it is up
        #[arg(short, long, default_value = "true")]
        repeating: bool,
    },
    Remove {
        // Name of the timer to remove
        name: String,
    }
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    
    let ini_path = match &args.config {
        Some(config) => config.clone(),
        None => {
            let mut path = std::env::current_exe().unwrap();
            path.pop();
            path.push("config.ini");
            
            if !path.clone().exists() {
                std::fs::write(&path, "")?;
            }
            path
        },
    };

    let mut conf = Ini::load_from_file(&ini_path)?;
    
    let timers = load_timers_from_file(&conf);
    match args.command {
        Some(Commands::Start) => start_timers(timers),
        Some(Commands::List) => list_timers(timers),
        Some(Commands::Add {
            name,
            notification,
            interval,
            repeating,
        }) => add_timer(&mut conf, name, notification, interval, repeating),
        Some(Commands::Remove {
            name,
        }) => remove_timer(&mut conf, name),
        None => return Ok(()),
    }

    // make the changes permanent
    conf.write_to_file(&ini_path)?;

    Ok(())
}

fn load_timers_from_file(ini: &Ini) -> Vec<Timer> {
    let mut timers: Vec<Timer> = Vec::new();
    for section in ini.sections() {
        if let Some(section) = section {
            let name = ini
                .get_from(Some(section), "name")
                .unwrap_or(section)
                .to_string();
            let notification = ini
                .get_from(Some(section), "notification")
                .unwrap_or(section)
                .to_string();
            let interval = ini.get_from(Some(section), "interval").unwrap_or("0");
            let interval = interval.parse::<u64>().unwrap();
            let repeating = ini.get_from(Some(section), "repeating").unwrap();
            let repeating = repeating.parse::<bool>().unwrap();
            timers.push(Timer {
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

fn list_timers(timers: Vec<Timer>) {
    for timer in timers {
        println!(
            "{}: {} {} {} seconds",
            timer.name,
            timer.notification,
            if timer.repeating { "every" } else { "after" },
            timer.interval.as_secs()
        );
    }
}

fn add_timer(ini: &mut Ini, name: String, notification: String, interval: u64, repeating: bool) {
    ini.with_section(Some(name))
        .set("notification", notification)
        .set("interval", interval.to_string())
        .set("repeating", repeating.to_string());
}

fn remove_timer(ini: &mut Ini, name: String) {
    ini.delete(Some(name));
}