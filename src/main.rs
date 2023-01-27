use std::{num::ParseIntError, thread};
use ini::Ini;

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

fn main() -> Result<(), ParseIntError> {
    let conf = Ini::load_from_file("config.ini").expect("couldnt find config.ini");
    let mut timers: Vec<Timer> = Vec::new();
    for section in conf.sections() {
        if let Some(section) = section {
            let name = conf.get_from(Some(section), "name").unwrap_or(section).to_string();
            let notification = conf.get_from(Some(section), "notification").unwrap_or(section).to_string();
            let interval = conf.get_from(Some(section), "interval").unwrap_or("0");
            let interval = interval.parse::<u64>().unwrap();
            let repeating = conf.get_from(Some(section), "repeating").unwrap();
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

    Ok(())
    
}
