//! # Hanging on by a Thread: main.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! The entry point to our program. This file holds the logic for
//! taking command line arguments, logging to either the terminal
//! or to a file, then setting up all structures and threads.
//!
//! ## Dependencies
//! This module depends on the following external crates:
//! - use std::{env, process}
//! - use std::sync::{Arc, Condvar, Mutex}
//! - use std::thread
//! - use std::time::Duration
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling


pub mod steward;
pub mod dragonrider;
pub mod stronghold;
mod depot;
pub mod dragondepot;
mod logger;

use std::{env, process, sync::{Arc, Mutex, Condvar}, thread, time::Duration};
use depot::Depot;
use logger::Logger;
use steward::Steward;
use dragonrider::DragonRider;
use stronghold::Stronghold;
use crate::dragondepot::DragonDepot;

/// Constant that contains the name of the file to log to when running
const LOG_FILE:&str = "log.txt";

/// The entry point for our program
///
/// Sets up logic for taking command line arguments,
/// logging to either the terminal or a file,
/// setting up the different structures,
/// then creating all the threads,
/// then running until ctrl + c is pressed or until time runs out.
///
fn main() {
    let args:Vec<String> = env::args().collect();
    // If size of arguments is not equal to 3, prints usage statement and exits program
    if args.len() != 3 {
        println!("Usage: cargo run <seconds_to_run> <T|F>");
        process::exit(1);
    }
    // Get number of seconds to run process for from first command line argument
    let seconds = get_seconds(&args[1]);
    // Determines if status information is printed to Stdout or to a log file
    let logger = get_logger(&args[2]);

    // A reference to the logger for printing status information
    let log_arc = Arc::new(Mutex::new(logger));

    // Depot where the steward stores resources to/the dragon riders take from
    let depot = Arc::new(Mutex::new(Depot::new()));

    // Signal for steward to collect supplies after stronghold is finished
    let steward_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from steward that burnstone is supplied in the depot
    let burnstone_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from steward that seaplum is supplied in the depot
    let seaplum_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from steward that klah is supplied in the depot
    let klah_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from dragon rider's depot that supplies for burnstone stronghold is ready
    let burnstone_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from dragon rider's depot that supplies for seaplum stronghold is ready
    let seaplum_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));
    // Signal from dragon rider's depot that supplies for klah stronghold is ready
    let klah_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));

    // Mini depot that dragon riders put resources in to know which stronghold to deliever to
    let dragon_depot = Arc::new(Mutex::new(DragonDepot::new(
        Arc::clone(&burnstone_stronghold_signal), Arc::clone(&seaplum_stronghold_signal),
        Arc::clone(&klah_stronghold_signal)
    )));

    // Steward
    let mut steward = Steward::new(
        Arc::clone(&depot), Arc::clone(&steward_signal), Arc::clone(&burnstone_signal),
        Arc::clone(&seaplum_signal), Arc::clone(&klah_signal), Arc::clone(&log_arc)
    );

    // Burnstone Stronghold
    let burnstone_stronghold = Stronghold::new(
        "Burnstone".to_string(), Arc::clone(&steward_signal),
        Arc::clone(&burnstone_stronghold_signal), Arc::clone(&log_arc)
    );
    // Seaplum Stronghold
    let seaplum_stronghold = Stronghold::new(
        "Seaplum".to_string(), Arc::clone(&steward_signal),
        Arc::clone(&seaplum_stronghold_signal), Arc::clone(&log_arc)
    );
    // Klah Stronghold
    let klah_stronghold = Stronghold::new(
        "Klah".to_string(), Arc::clone(&steward_signal), Arc::clone(&klah_stronghold_signal),
        Arc::clone(&log_arc)
    );

    // Dragon Rider for Burnstone resource
    let mut burnstone_dragon_rider = DragonRider::new(
        "Burnstone".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&burnstone_signal), Arc::clone(&log_arc)
    );
    // Dragon Rider for Seaplum resource
    let mut seaplum_dragon_rider = DragonRider::new(
        "Seaplum".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&seaplum_signal), Arc::clone(&log_arc)
    );
    // Dragon Rider for Klah resource
    let mut klah_dragon_rider = DragonRider::new(
        "Klah".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&klah_signal), Arc::clone(&log_arc)
    );

    // Steward thread
    thread::spawn(move || {
        steward.go();
    });

    // Stronghold threads
    thread::spawn(move || {
        burnstone_stronghold.go();
    });
    thread::spawn(move || {
        seaplum_stronghold.go();
    });
    thread::spawn(move || {
        klah_stronghold.go();
    });

    // Dragon rider threads
    thread::spawn(move || {
        burnstone_dragon_rider.go();
    });
    thread::spawn(move || {
        seaplum_dragon_rider.go();
    });
    thread::spawn(move || {
        klah_dragon_rider.go();
    });

    // Determines if process waits for a number amount of seconds or to run indefinitely
    if seconds > 0 {
        // Waits for inputted seconds before quiting
        thread::sleep(Duration::from_secs(seconds as u64));
    } else {
        // Runs forever until user presses Ctrl C to kill process
        thread::park();
    }

}

/// Get the number of seconds given by an user.
/// 
/// Prints an error message and exits out of the program if user inputted a non integer datatype
/// as the first command line argument.
/// 
/// # Arguments
/// * `argument`: The first command line argument inputted by an user.
/// 
/// # Return
/// An integer that represents what the amount of seconds that the user entered.
/// 
/// # Panics
/// The program exits if the argument cannot be parsed correctly.
fn get_seconds(argument:&String) -> i64 {
    let seconds_convert = argument.parse::<i64>();
    if seconds_convert.is_err() {
        eprintln!("Invalid argument for number of seconds: must be an integer");
        process::exit(1);
    }
    seconds_convert.unwrap()
}

/// Returns a logger that either prints to Stdout or to the log file.
/// 
/// Prints error messages and exits out of the program if user inputted an incorrect argument
/// or if an occurred when creating the logger.
/// 
/// # Arguments
/// * `argument`: The second command line argument inputted by an user.
/// 
/// # Return
/// A logger that prints to Stdout if the user inputted a "F" as their argument or a logger that
/// prints to a log file if the user inputted a "T" as their argument.
/// 
/// # Panics
/// The program exits if the user supplied an argument that isn't a T or a F and if an error 
/// occurred when creating the log file.
fn get_logger(argument:&String) -> Logger {
    if argument.as_str() != "T" && argument.as_str() != "F" {
        eprintln!("Invalid argument for true or false condition for logging");
        process::exit(1);
    }
    let is_writable = if argument.as_str() == "T" { true } else { false };
    let logger_result = Logger::new(LOG_FILE.to_string(), is_writable);
    if logger_result.is_err() {
        eprintln!("File does not exist for logger to write to");
        process::exit(1);
    }
    logger_result.unwrap()
}
