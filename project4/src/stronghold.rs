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
//! This module relies on the Rust standard library's synchronization primitives and threading support:
//! - `std::sync::{Arc, Condvar, Mutex}` for thread-safe handling of signals and shared state.
//! - `std::thread` for simulating concurrent operations.
//! - `std::time::Duration` for managing operation delays.
//! - `rand::{thread_rng, Rng}` for generating random intervals for resource distribution and consumption.
//! - `crate::logger::Logger` for logging status and operation messages.
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

/// Minimum time duration for resource handling operations.
const MIN_SECONDS:f64 = 5.0;

/// Represents a stronghold that manages resources within the system.
///
/// # Fields
/// - `name`: The name of the stronghold, usually related to the specific resource it manages.
/// - `resources_received`: A signal to notify the steward that resources have been successfully 
///    received.
/// - `resources_available`: A signal indicating that resources required by the stronghold are 
///    available for collection.
/// - `writer`: A logger for recording status updates and operations.
pub struct Stronghold {
    name: String,
    resources_received: Arc<(Mutex<bool>, Condvar)>,
    resources_available: Arc<(Mutex<bool>, Condvar)>,
    writer: Arc<Mutex<Logger>>
}

impl Stronghold {
    /// Constructs a new `Stronghold`.
    /// 
    /// # Arguments
    /// * `name`: The name of the stronghold and the main resource it harvests.
    /// * `resources_received`: The signal to notify the steward that resources have been 
    ///    successfully received.
    /// * `resources_available`: The signal to notify stronghold that its resources are available.
    /// * `writer`: The logger to write status info to.
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

    /// Waits for notification that the necessary resources are available at the depot.
    fn wait_for_resources(&self) {
        let (lock, condvar) = &*self.resources_available;
        let guard = lock.lock().unwrap();
        self.write_status(self.waiting());
        let mut guard = condvar.wait_while(guard, |condition| {
           !*condition 
        }).unwrap();
        self.write_status(self.received());
        *guard = false;
    }

    /// Returns a message indicating the stronghold is waiting for resources.
    fn waiting(&self) -> String {
        let mut message = "Stronghold ".to_string() + self.name.clone().as_str();
        message = message + " waiting for its resources";
        message
    }

    /// Writes a status message to the logger.
    /// 
    /// # Arguments
    /// * `message`: The message that is being written to the logger.
    fn write_status(&self, message:String) {
        let lock = &*self.writer;
        let mut writer = lock.lock().unwrap();
        writer.write(message);
    }

    /// Returns a message indicating that resources have been received.
    fn received(&self) -> String {
        let mut message = "Dragon riders had delievered resources to Stronghold ".to_string();
        message = message + self.name.clone().as_str();
        message
    }

    /// Notifies that resources have been received.
    fn resources_received(&self) {
        let (lock, condvar) = &*self.resources_received;
        let mut received = lock.lock().unwrap();
        *received = true;
        condvar.notify_one();
    }

    /// Distributes resources within the stronghold.
    /// 
    /// Waits for a random amount of time (between 5 to 9 seconds).
    fn distribute_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        self.write_status(self.distribute_or_consume(true, false));
        thread::sleep(time);
        self.write_status(self.distribute_or_consume(true, true));
    }

    /// Returns a status message for distributing or consuming resources.
    /// 
    /// # Arguments
    /// * `distributing`: Boolean that determines if the stronghold is distributing or consuming
    ///    with true representing that the stronghold is distributing.
    /// * `finished`: Boolean that determines if the stronghold has started or finished with true
    ///    representing that the stronghold has finished.
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

    /// Consumes resources within the stronghold.
    /// 
    /// Waits for a random amount of time (between 5 to 9 seconds).
    fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        self.write_status(self.distribute_or_consume(false, false));
        thread::sleep(time);
        self.write_status(self.distribute_or_consume(false, true));
    }

    /// Executes the full cycle of resource handling from waiting to consumption.
    pub fn go(&self) {
        loop {
            self.wait_for_resources();
            self.resources_received();
            self.distribute_resources();
            self.consume_resources();
        }
    }

}