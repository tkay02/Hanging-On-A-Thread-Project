//! # Hanging on by a Thread: main.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//!
//!
//! ## Dependencies
//!
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

use std::{env, process};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use depot::Depot;
use steward::Steward;

use dragonrider::DragonRider;
use stronghold::Stronghold;

use crate::dragondepot::DragonDepot;

const LOG_FILE:&str = "log.txt";

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
        Arc::clone(&seaplum_signal), Arc::clone(&klah_signal)
    );

    //Burnstone Stronghold
    let burnstone_stronghold = Stronghold::new(
        "Burnstone".to_string(), Arc::clone(&steward_signal), 
        Arc::clone(&burnstone_stronghold_signal)
    );
    //Seaplum Stronghold
    let seaplum_stronghold = Stronghold::new(
        "Seaplum".to_string(), Arc::clone(&steward_signal), Arc::clone(&seaplum_stronghold_signal)
    );
    //Klah Stronghold
    let klah_stronghold = Stronghold::new(
        "Klah".to_string(), Arc::clone(&steward_signal), Arc::clone(&klah_stronghold_signal)
    );

    //Dragon Rider for Burnstone resource
    let mut burnstone_dragon_rider = DragonRider::new(
        "Burnstone".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&burnstone_signal)
    );
    //Dragon Rider for Seaplum resource
    let mut seaplum_dragon_rider = DragonRider::new(
        "Seaplum".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&seaplum_signal)
    );
    //Dragon Rider for Klah resource
    let mut klah_dragon_rider = DragonRider::new(
        "Klah".to_string(), Arc::clone(&depot), Arc::clone(&dragon_depot),
        Arc::clone(&klah_signal)
    );

    //Steward thread
    thread::spawn(move || {
        for _ in 0..2 {
            steward.produce();
            steward.wait_for_received();
        }
    });

    //Stronghold threads
    thread::spawn(move || {
        for _ in 0..2 {
            burnstone_stronghold.wait_for_resources();
            burnstone_stronghold.resources_received();
            burnstone_stronghold.distribute_resources();
            burnstone_stronghold.consume_resources();
        }
    });
    thread::spawn(move || {
        for _ in 0..2 {
            seaplum_stronghold.wait_for_resources();
            seaplum_stronghold.resources_received();
            seaplum_stronghold.distribute_resources();
            seaplum_stronghold.consume_resources();
        }
    });
    thread::spawn(move || {
        for _ in 0..2 {
            klah_stronghold.wait_for_resources();
            klah_stronghold.resources_received();
            klah_stronghold.distribute_resources();
            klah_stronghold.consume_resources();
        }
    });

    //Dragon rider threads
    thread::spawn(move || {
        for _ in 0..2 {
            burnstone_dragon_rider.wait_for_consumation();
            burnstone_dragon_rider.consume();
            burnstone_dragon_rider.group_resources();
        }
    });
    thread::spawn(move || {
        for _ in 0..2 {
            seaplum_dragon_rider.wait_for_consumation();
            seaplum_dragon_rider.consume();
            seaplum_dragon_rider.group_resources();
        }
    });
    thread::spawn(move || {
        for _ in 0..2 {
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
