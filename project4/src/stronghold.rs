//! # Hanging on by a Thread: stronghold.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! Sets up the structure for strongholds that has:
//! - The name of the stronghold (which contains the resource)
//! - A signal to tell the steward that supplies have been successfully received
//! - A signal to receive that the resources that the stronghold is lacking is available
//! The strongholds also have the functionality of waiting and receiving resources. Then it
//! has the ability to distribute and consume resources.
//!
//! ## Dependencies
//! This module depends on the following external crate:
//! - use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};
//! - use rand::{thread_rng, Rng};
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};
use rand::{thread_rng, Rng};
use crate::logger::Logger;

const MIN_SECONDS:f64 = 5.0;

//Need a reference to a mutex
pub struct Stronghold {
    // The name of the stronghold (which contains the resource)
    name: String,
    // Signal to tell steward that supplies have been successfully received
    resources_received: Arc<(Mutex<bool>, Condvar)>,
    // Signal to receive that the resources that the stronghold is lacking is available
    resources_available: Arc<(Mutex<bool>, Condvar)>,
    // Write status to Stdout or to a file
    writer: Arc<Mutex<Logger>>
}

impl Stronghold {

    pub fn new(name: String,
               resources_received: Arc<(Mutex<bool>, Condvar)>,
               resources_available: Arc<(Mutex<bool>, Condvar)>,
               writer: Arc<Mutex<Logger>>) -> Stronghold {
        Stronghold {
            name,
            resources_received,
            resources_available,
            writer
        }
    }

    pub fn wait_for_resources(&self) {
        let (lock, condvar) = &*self.resources_available;
        let guard = lock.lock().unwrap();
        self.write_status(self.waiting());
        let mut guard = condvar.wait_while(guard, |condition| {
           !*condition 
        }).unwrap();
        self.write_status(self.received());
        *guard = false;
    }

    pub fn waiting(&self) -> String {
        let mut message = "Stronghold ".to_string() + self.name.clone().as_str();
        message = message + " waiting for its resources";
        message
    }

    fn write_status(&self, message:String) {
        let lock = &*self.writer;
        let writer = lock.lock().unwrap();
        writer.write(message);
    }

    pub fn received(&self) -> String {
        let mut message = "Dragon riders had delievered resources to Stronghold ".to_string();
        message = message + self.name.clone().as_str();
        message
    }
    
    pub fn resources_received(&self) {
        let (lock, condvar) = &*self.resources_received;
        let mut received = lock.lock().unwrap();
        *received = true;
        condvar.notify_one();
    }
    
    pub fn distribute_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        self.write_status(self.distribute_or_consume(true, false));
        thread::sleep(time);
        self.write_status(self.distribute_or_consume(true, true));
    }

    fn distribute_or_consume(&self, distributing:bool, finished:bool) -> String {
        let message = "Stronghold ".to_string() + self.name.clone().as_str();
        if distributing {
            if finished {
                message + " has finished distributing resources"
            } else {
                message + " is now distributing resources"
            }
        } else {
            if finished {
                message + " has finished consuming resources"
            } else {
                message + " is now consuming resources"
            }
        }
    }

    //Maybe make methods that returns strings?

    pub fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        self.write_status(self.distribute_or_consume(false, false));
        thread::sleep(time);
        self.write_status(self.distribute_or_consume(false, true));
    }

    pub fn go(&mut self) {
        self.wait_for_resources();
        self.resources_received();
        self.distribute_resources();
        self.consume_resources();
    }

}