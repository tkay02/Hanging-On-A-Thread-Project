//! # Hanging on by a Thread: stronghold.rs
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

use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};

use rand::{thread_rng, Rng};

const MIN_SECONDS:f64 = 5.0;

//Need a reference to a mutex
pub struct Stronghold {
    // The name of the stronghold (which contains the resource)
    name: String,
    // Signal to tell steward that supplies have been successfully received
    resources_received: Arc<(Mutex<bool>, Condvar)>,
    // Signal to receive that the resources that the stronghold is lacking is available
    resources_available: Arc<(Mutex<bool>, Condvar)>
}

impl Stronghold {

    pub fn new(name: String,
               resources_received: Arc<(Mutex<bool>, Condvar)>,
               resources_available: Arc<(Mutex<bool>, Condvar)>) -> Stronghold {
        Stronghold {
            name: name,
            resources_received: resources_received,
            resources_available: resources_available
        }
    }

    pub fn wait_for_resources(&self) {
        let (lock, condvar) = &*self.resources_available;
        let guard = lock.lock().unwrap();
        println!("{}", self.waiting());
        let mut guard = condvar.wait_while(guard, |condition| {
           !*condition 
        }).unwrap();
        println!("{}", self.received());
        *guard = false;
    }

    pub fn waiting(&self) -> String {
        let mut message = "Stronghold ".to_string() + self.name.clone().as_str();
        message = message + " waiting for its resources";
        message
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
        println!("Stronghold {} is now distributing resources", self.name);
        thread::sleep(time);
        println!("Stronghold {} has finished distributing resources", self.name);
    }

    //Maybe make methods that returns strings?

    pub fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now consuming resources", self.name);
        thread::sleep(time);
        println!("Stronghold {} has finished consuming resources", self.name);
    }

    pub fn go(&mut self) {
        self.wait_for_resources();
        self.resources_received();
        self.distribute_resources();
        self.consume_resources();
    }

}