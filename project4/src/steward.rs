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
//! - `std::sync::{Arc, Condvar, Mutex, MutexGuard}`: Used for thread-safe sharing and synchronization.
//! - `rand::{thread_rng, Rng}`: Used to randomly select resources to collect and deliver.
//! - `crate::{depot::Depot, logger::Logger}`: Dependencies within the project for managing resources and logging.
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::sync::{Arc, Condvar, Mutex, MutexGuard};
use rand::{thread_rng, Rng};

use crate::{depot::Depot, logger::Logger};

/// The total number of different resources
const MAX_RESOURCES:f64 = 3.0;

/// The types of resources
pub const RESOURCES:[&'static str; 3] = ["Burnstone", "Seaplum", "Klah"];

/// Represents the steward responsible for managing resource distribution.
///
/// # Fields
/// - `depot`: Shared memory of the central resource depot.
/// - `stronghold_received`: Signal from strongholds when resources have been collected.
/// - `firestone_ready`: Signal that firestone is ready for delivery to the depot.
/// - `seaplum_ready`: Signal that seaplum is ready for delivery.
/// - `klah_ready`: Signal that klah is ready for delivery.
/// - `writer`: Logger for outputting status information.
/// - `resource1`: First type of resource being handled during the cycle.
/// - `resource2`: Second type of resource being handled.
pub struct Steward {
    depot: Arc<Mutex<Depot>>,
    stronghold_received: Arc<(Mutex<bool>, Condvar)>,
    firestone_ready: Arc<(Mutex<bool>, Condvar)>,
    seaplum_ready: Arc<(Mutex<bool>, Condvar)>,
    klah_ready: Arc<(Mutex<bool>, Condvar)>,
    writer: Arc<Mutex<Logger>>,
    resource1: String,
    resource2: String
}

impl Steward {
    /// Constructs a new `Steward`.
    pub fn new(depot:Arc<Mutex<Depot>>, 
               stronghold:Arc<(Mutex<bool>, Condvar)>,
               firestone:Arc<(Mutex<bool>, Condvar)>, 
               seaplum:Arc<(Mutex<bool>, Condvar)>,
               klah:Arc<(Mutex<bool>, Condvar)>,
               writer:Arc<Mutex<Logger>>) -> Steward {
        Steward {
            depot,
            stronghold_received: stronghold,
            firestone_ready: firestone,
            seaplum_ready: seaplum,
            klah_ready: klah,
            writer,
            resource1: String::new(),
            resource2: String::new()
        }
    }
    /// Collects resources randomly to be delivered to the depot.
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

    /// Manages the production and delivery of resources to the depot.
    pub fn produce(&mut self) {
        self.collect_resources();
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        self.resource_ready(self.resource1.clone(), &mut depot);
        self.resource_ready(self.resource2.clone(), &mut depot);
        self.write_status(self.resources_delievered());
    }

    /// Returns a status message detailing the resources delivered.
    pub fn resources_delievered(&self) -> String {
        let mut message = "The steward has delievered resources ".to_string();
        message = message + self.resource1.clone().as_str() + " and " + 
                  self.resource2.clone().as_mut_str() + " to the depot";
        message
    }

    /// Returns a string message that the Steward is waiting for strongholds to collect supplies.
    pub fn waiting(&self) -> String {
        "The steward is waiting for stronghold to collect supplies".to_string()
    }

    /// Returns a string message that the Steward is ready to collect more resources.
    pub fn finished_waiting(&self) -> String {
        "Steward is now ready to collect resources to give to the depot".to_string()
    }

    /// Outputs a status message to the logger.
    fn write_status(&self, message:String) {
        let lock = &*self.writer;
        let mut writer = lock.lock().unwrap();
        writer.write(message);
    }

    /// Helper method to signal the depot that a resource is ready.
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
                depot.place_klah();
                let (lock2, condvar) = &*self.klah_ready;
                let mut ready = lock2.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            },
            _ => { unreachable!() }
        }
    }

    /// Waits for a signal from strongholds indicating that they have received the resources.
    pub fn wait_for_received(&self) {
        let (lock, condvar) = &*self.stronghold_received;
        let guard = lock.lock().unwrap();
        self.write_status(self.waiting());
        let mut guard = condvar.wait_while(guard, |condition| {
            !*condition
        }).unwrap();
        self.write_status(self.finished_waiting());
        *guard = false;
    }

    /// Orchestrates the complete cycle of resource handling from collection to delivery.
    pub fn go(&mut self) {
        self.produce();
        self.wait_for_received();
    }
}