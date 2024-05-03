//! # Hanging on by a Thread: dragonrider.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! Sets up structure for the dragon rider which holds references to
//! - the type of resource that the dragonrider is carrying
//! - the depot to obtain resources
//! - the depot to store resources obtained by the dragonriders
//! and has a signal that the depot has resources that are ready to be collected
//! The dragon rider has the capability to wait and obtain resources, wait and consume resources,
//! and group resources.
//!
//! ## Dependencies
//! This module depends on the `Depot` and `DragonDepot` for resource management, and uses
//! synchronization primitives from the Rust standard library (`Arc`, `Mutex`, `Condvar`).
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::sync::{Arc, Condvar, Mutex};
use crate::{depot::Depot, dragondepot::DragonDepot};
use crate::logger::Logger;

/// Structure that represents a Dragon Rider that will carry resources to the depot
///
/// # Fields
/// - `resource_type`: The type of resource that the dragonrider is carrying
/// - `depot`: A reference to the depot to obtain resources from
/// - `dragon_depot`: A reference to the dragon depot that stores resources from the dragonriders
/// - `depot_signal`: A signal that the depot has resources that are ready to be collected
/// - `writer`: Used to print status onto Stdout or a file
pub struct DragonRider {
    resource_type: String,
    depot: Arc<Mutex<Depot>>,
    dragon_depot: Arc<Mutex<DragonDepot>>,
    depot_signal: Arc<(Mutex<bool>, Condvar)>,
    writer: Arc<Mutex<Logger>>
}

impl DragonRider {
    /// Constructs a new `DragonRider` instance with the ability to obtain
    /// and deliver resources.
    pub fn new(resource:String, 
               depot:Arc<Mutex<Depot>>,
               dragon_depot:Arc<Mutex<DragonDepot>>,
               depot_signal:Arc<(Mutex<bool>, Condvar)>,
               writer:Arc<Mutex<Logger>>) -> DragonRider {
        DragonRider {
            resource_type: resource,
            depot,
            dragon_depot,
            depot_signal,
            writer
        }
    }

    /// Logs a message indicating the Dragon Rider is waiting for a resource to become available.
    pub fn waiting_for_resource(&self) -> String {
        self.resource_type.clone() + " dragon rider is waiting for resource"
    }

    /// Logs a message indicating the Dragon Rider has obtained the resource.
    pub fn obtained_resource(&self) -> String {
        self.resource_type.clone() + " dragon rider has obtained resource"
    }

    /// Retrieves a resource from the main depot,
    /// based on the type of resource the Dragon Rider handles.
    pub fn consume(&self) {
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        match self.resource_type.as_str() {
            "Burnstone" => {
                let _ = &depot.take_burnstone();
            },
            "Seaplum" => {
                let _ = &depot.take_seaplum();
            },
            "Klah" => {
                let _ = &depot.take_klah();
            }
            _ => { unreachable!() }
        }
        self.write_status(self.obtained_resource());
    }

    /// Writes a status message to the logger.
    fn write_status(&self, message:String) {
        let lock = &*self.writer;
        let mut writer = lock.lock().unwrap();
        writer.write(message);
    }

    /// Waits for a signal that indicates resources are ready for consumption.
    pub fn wait_for_consumation(&self) {
        let (lock, condvar) = &*self.depot_signal;
        let guard = lock.lock().unwrap();
        self.write_status(self.waiting_for_resource());
        let mut guard = condvar.wait_while(guard, |condition| {
            !*condition
        }).unwrap();
        *guard = false;
    }

    /// Places the obtained resource into the dragon depot.
    pub fn group_resources(&mut self) {
        let lock = &*self.dragon_depot;
        let mut dragon_depot = lock.lock().unwrap();
        dragon_depot.place_resource(self.resource_type.clone());
    }

    /// Main operation flow of the Dragon Rider; coordinates waiting, consuming, and grouping resources.
    pub fn go(&mut self) {
        self.wait_for_consumation();
        self.consume();
        self.group_resources();
    }

}