use std::env;
use std::process;

use evolvingstring::EvolvingString;
use chrono::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <subcommand> [args]", args[0]);
        eprintln!("Subcommands:");
        eprintln!("  init <initial_string> <shared_secret> <interval_seconds>");
        eprintln!("  current <initial_string> <shared_secret> <interval_seconds>");
        eprintln!("  predict <initial_string> <shared_secret> <interval_seconds> <datetime>");
        process::exit(1);
    }

    let subcommand = &args[1].to_lowercase();

    match subcommand.as_str() {
        "init" => {
            if args.len() != 5 {
                eprintln!("Usage: init <initial_string> <shared_secret> <interval_seconds>");
                process::exit(1);
            }
            let initial_string = &args[2];
            let shared_secret = &args[3];
            let interval_seconds: u64 = args[4].parse().unwrap();

            let evolving_string = EvolvingString::new(
                initial_string.to_string(),
                shared_secret.to_string(),
                interval_seconds,
            );
            println!("{}", evolving_string.to_base64());
        }
        "current" => {
            if args.len() != 5 {
                eprintln!("Usage: current <initial_string> <shared_secret> <interval_seconds>");
                process::exit(1);
            }
            let initial_string = &args[2];
            let shared_secret = &args[3];
            let interval_seconds: u64 = args[4].parse().expect("Failed to parse interval.");

            let evolving_string = EvolvingString::new(initial_string.to_string(), shared_secret.to_string(), interval_seconds);

            println!("Current string: {}", evolving_string.current());
        }
        "predict" => {
            if args.len() != 6 {
                eprintln!("Usage: predict <initial_string> <shared_secret> <interval_seconds> <datetime>");
                process::exit(1);
            }
            let initial_string = &args[2];
            let shared_secret = &args[3];
            let interval_seconds: u64 = args[4].parse().expect("Failed to parse interval.");
            let datetime_str = &args[5];

            let target_time = match DateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S%z") {
                Ok(dt) => dt,
                Err(_) => {
                    eprintln!("Failed to parse datetime from string.");
                    process::exit(1);
                }
            };
            let now = Utc::now();

            if target_time < now {
                eprintln!("Predicted datetime must be in the future");
                process::exit(1);
            }

            let duration_to_target = target_time.signed_duration_since(now);
            let future_seconds = duration_to_target.num_seconds() as u64;
            let evolving_string = EvolvingString::new(initial_string.to_string(), shared_secret.to_string(), interval_seconds);

            println!("Predicted string at {}: {}", datetime_str, evolving_string.predict(future_seconds));
        }
        _ => {
            eprintln!("Unknown subcommand: {}", subcommand);
            process::exit(1);
        }
    }
}
