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

use std::{env, process};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use depot::Depot;
use logger::Logger;
use steward::Steward;

use dragonrider::DragonRider;
use stronghold::Stronghold;

use crate::dragondepot::DragonDepot;

const LOG_FILE:&str = "log.txt";

/// The entry point for our program
///
/// Sets up logic for taking command line arguments,
/// logging to either the terminal or a file,
/// setting up the different structures,
/// then creating all the threads,
/// then running until ctrl + c is pressed
///
fn main() {
    
    let args:Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run <seconds_to_run> <T|F>");
        process::exit(1);
    }
    let seconds_arg = &args[1];
    let log_arg = &args[2];
    let seconds_convert = seconds_arg.parse::<i64>();
    if seconds_convert.is_err() {
        println!("Invalid argument for number of seconds: must be an integer");
        process::exit(1);
    }
    let seconds = seconds_convert.unwrap();
    if log_arg.as_str() != "T" && log_arg.as_str() != "F" {
        println!("Invalid argument for true or false condition for logging");
        process::exit(1);
    }
    //Insert code for log file creation
    let is_writable = if log_arg.as_str() == "T" { true } else { false };
    let logger_result = Logger::new(LOG_FILE.to_string(), is_writable);
    if logger_result.is_err() {
        println!("File does not exist for logger to write to");
        process::exit(1);
    }
    let logger = logger_result.unwrap();
    let log_arc = Arc::new(Mutex::new(logger));

    //Depot where the steward stores resources to/the dragon riders take from
    let depot = Arc::new(Mutex::new(Depot::new()));

    //Signal for steward to collect supplies after stronghold is finished
    let steward_signal = Arc::new((Mutex::new(false), Condvar::new()));
    //Signal from steward that burnstone is supplied in the depot
    let burnstone_signal = Arc::new((Mutex::new(false), Condvar::new()));
    //Signal from steward that seaplum is supplied in the depot
    let seaplum_signal = Arc::new((Mutex::new(false), Condvar::new()));
    //Signal from steward that klah is supplied in the depot
    let klah_signal = Arc::new((Mutex::new(false), Condvar::new())); 
    //Signal from dragon rider's depot that supplies for burnstone stronghold is ready
    let burnstone_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));
    //Signal from dragon rider's depot that supplies for seaplum stronghold is ready
    let seaplum_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));
    //Signal from dragon rider's depot that supplies for klah stronghold is ready
    let klah_stronghold_signal = Arc::new((Mutex::new(false), Condvar::new()));

    //Mini depot that dragon riders put resources in to know which stronghold to deliever to
    let dragon_depot = Arc::new(Mutex::new(DragonDepot::new(
        Arc::clone(&burnstone_stronghold_signal), Arc::clone(&seaplum_stronghold_signal),
        Arc::clone(&klah_stronghold_signal)
    )));

    //Steward
    let mut steward = Steward::new(
        Arc::clone(&depot), Arc::clone(&steward_signal), Arc::clone(&burnstone_signal),
        Arc::clone(&seaplum_signal), Arc::clone(&klah_signal), Arc::clone(&log_arc)
    );

    //Burnstone Stronghold
    let burnstone_stronghold = Stronghold::new(
        "Burnstone".to_string(), Arc::clone(&steward_signal), 
        Arc::clone(&burnstone_stronghold_signal), Arc::clone(&log_arc)
    );
    //Seaplum Stronghold
    let seaplum_stronghold = Stronghold::new(
        "Seaplum".to_string(), Arc::clone(&steward_signal), 
        Arc::clone(&seaplum_stronghold_signal), Arc::clone(&log_arc)
    );
    //Klah Stronghold
    let klah_stronghold = Stronghold::new(
        "Klah".to_string(), Arc::clone(&steward_signal), Arc::clone(&klah_stronghold_signal),
        Arc::clone(&log_arc)
    );

    //Dragon Rider for Burnstone resource
    let mut burnstone_dragon_rider = DragonRider::new(
        "Burnstone".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&burnstone_signal), Arc::clone(&log_arc)
    );
    //Dragon Rider for Seaplum resource
    let mut seaplum_dragon_rider = DragonRider::new(
        "Seaplum".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&seaplum_signal), Arc::clone(&log_arc)
    );
    //Dragon Rider for Klah resource
    let mut klah_dragon_rider = DragonRider::new(
        "Klah".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&klah_signal), Arc::clone(&log_arc)
    );

    //Steward thread
    thread::spawn(move || {
        loop {
            steward.produce();
            steward.wait_for_received();
        }
    });

    //Stronghold threads
    thread::spawn(move || {
        loop {
            burnstone_stronghold.wait_for_resources();
            burnstone_stronghold.resources_received();
            burnstone_stronghold.distribute_resources();
            burnstone_stronghold.consume_resources();
        }
    });
    thread::spawn(move || {
        loop {
            seaplum_stronghold.wait_for_resources();
            seaplum_stronghold.resources_received();
            seaplum_stronghold.distribute_resources();
            seaplum_stronghold.consume_resources();
        }
    });
    thread::spawn(move || {
        loop {
            klah_stronghold.wait_for_resources();
            klah_stronghold.resources_received();
            klah_stronghold.distribute_resources();
            klah_stronghold.consume_resources();
        }
    });

    //Dragon rider threads
    thread::spawn(move || {
        loop {
            burnstone_dragon_rider.wait_for_consumation();
            burnstone_dragon_rider.consume();
            burnstone_dragon_rider.group_resources();
        }
    });
    thread::spawn(move || {
        loop {
            seaplum_dragon_rider.wait_for_consumation();
            seaplum_dragon_rider.consume();
            seaplum_dragon_rider.group_resources();
        }
    });
    thread::spawn(move || {
        loop {
            klah_dragon_rider.wait_for_consumation();
            klah_dragon_rider.consume();
            klah_dragon_rider.group_resources();
        }
    });


    if seconds > 0 {
        // Waits for inputted seconds before quiting
        thread::sleep(Duration::from_secs(seconds as u64));
    } else {
        // Runs forever until user presses Ctrl C to kill process
        thread::park();
    }

}
