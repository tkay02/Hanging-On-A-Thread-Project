//! # Hanging on by a Thread: steward.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! Sets up the structure for the Steward which holds references to:
//! - shared memory of depot
//! - both resources
//! It also holds signals for:
//! - receiving from a stronghold
//! - message that firestone is ready to be delivered
//! - message that seaplum is ready to be delivered
//! - message that klah is ready to be delivered
//! The Steward collects, receives, produces, and delivers resources
//!
//! ## Dependencies
//! This module depends on the following external crate:
//! - use std::sync::{Arc, Condvar, Mutex, MutexGuard};
//! - use rand::{thread_rng, Rng};
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use rand::{thread_rng, Rng};

use crate::depot::Depot;

const MAX_RESOURCES:f64 = 3.0;

pub const RESOURCES:[&'static str; 3] = ["Burnstone", "Seaplum", "Klah"];

/// Input fields for steward
pub struct Steward {
    // Reference to shared memory of depot
    depot: Arc<Mutex<Depot>>,
    // Signal to receive from a stronghold
    stronghold_received: Arc<(Mutex<bool>, Condvar)>,
    // Signal to send that firestone is ready to be delivered
    firestone_ready: Arc<(Mutex<bool>, Condvar)>,
    // Signal to send that seaplum is ready to be delivered
    seaplum_ready: Arc<(Mutex<bool>, Condvar)>,
    // Signal to send that klah is ready to be delivered
    klah_ready: Arc<(Mutex<bool>, Condvar)>,
    resource1: String,
    resource2: String
}

impl Steward {

    pub fn new(depot:Arc<Mutex<Depot>>, 
               stronghold:Arc<(Mutex<bool>, Condvar)>,
               firestone:Arc<(Mutex<bool>, Condvar)>, 
               seaplum:Arc<(Mutex<bool>, Condvar)>,
               klah:Arc<(Mutex<bool>, Condvar)>) -> Steward {
        Steward {
            depot: depot,
            stronghold_received: stronghold,
            firestone_ready: firestone,
            seaplum_ready: seaplum,
            klah_ready: klah,
            resource1: String::new(),
            resource2: String::new()
        }
    }

    fn collect_resources(&mut self) {
        let mut rng = thread_rng();
        let rng1 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        let mut rng2 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        while rng1 == rng2 {
            rng2 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        }
        self.resource1 = String::from(RESOURCES[rng1]);
        self.resource2 = String::from(RESOURCES[rng2]);
    }

    pub fn produce(&mut self) {
        self.collect_resources();
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        //self.resource_ready(self.resource1.clone(), &mut depot);
        match self.resource1.as_str() {
            "Burnstone" => {
                depot.place_burnstone();
                let (lock2, condvar) = &*self.firestone_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            "Seaplum" => {
                depot.place_seaplum();
                let (lock2, condvar) = &*self.seaplum_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            }
            "Klah" => {
                depot.place_kleh();
                let (lock2, condvar) = &*self.klah_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            _ => { unreachable!() }
        }
        //self.resource_ready(self.resource2.clone(), &mut depot);
        match self.resource2.as_str() {
            "Burnstone" => {
                depot.place_burnstone();
                let (lock2, condvar) = &*self.firestone_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            "Seaplum" => {
                depot.place_seaplum();
                let (lock2, condvar) = &*self.seaplum_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            }
            "Klah" => {
                depot.place_kleh();
                let (lock2, condvar) = &*self.klah_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            _ => { unreachable!() }
        }
        println!("{}", self.resources_delievered());
    }

    pub fn resources_delievered(&self) -> String {
        let mut message = "The steward has delievered resources ".to_string();
        message = message + self.resource1.clone().as_str() + " and " + 
                  self.resource2.clone().as_mut_str() + " to the depot";
        message
    }

    pub fn waiting(&self) -> String {
        "The steward is waiting for stronghold to collect supplies".to_string()
    }

    pub fn finished_waiting(&self) -> String {
        "Steward is now ready to collect resources to give to the depot".to_string()
    }

    // I hope this helper method works
    fn resource_ready(&self, resource:String, depot:&mut MutexGuard<Depot>) {
        match resource.as_str() {
            "Burnstone" => {
                depot.place_burnstone();
                let (lock2, condvar) = &*self.firestone_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            "Seaplum" => {
                depot.place_seaplum();
                let (lock2, condvar) = &*self.seaplum_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            }
            "Klah" => {
                depot.place_kleh();
                let (lock2, condvar) = &*self.klah_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            _ => { unreachable!() }
        }
    }

    

    pub fn wait_for_received(&self) {
        let (lock, condvar) = &*self.stronghold_received;
        let guard = lock.lock().unwrap();
        println!("{}", self.waiting());
        let mut guard = condvar.wait_while(guard, |condition| {
            !*condition
        }).unwrap();
        println!("{}", self.finished_waiting());
        *guard = false;
    }

    pub fn go(&mut self) {
        self.produce();
        self.wait_for_received();
    }


}