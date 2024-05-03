//! # Hanging on by a Thread: dragondepot.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! Sets up the dragon depot structure which holds:
//! - The collected items
//! - A count of the items
//! - Signals for each stronghold messaging that the resources are available
//! This file also checks for each resource. Then it places and depletes each resource.
//!
//! ## Dependencies
//! This module utilizes Rust's standard synchronization primitives from the `std::sync` package,
//! including `Arc`, `Mutex`, and `Condvar`, to ensure thread-safe operations across multiple
//! dragon riders and resource distribution handlers.
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::sync::{Arc,Mutex,Condvar};

/// Constant for the maximum amount of items allowed.
const MAX_ITEM:usize = 2;

/// Structure that represents a Depot for the dragon riders to interact with
///
/// # Fields
/// - `collected_item1`: First item collected by the dragon riders
/// - `collected_ietm2`: Second item collected by the dragon riders
/// - `item_count`: The amount of items stored in the dragon depot
/// - `burnstone_signal`: A signal for burnstone stronghold that its resources are available
/// - `seaplum_signal`: A signal for seaplum stronghold that its resources are available
/// - `klah_signal`: A signal for klah stronghold that its resources are available
pub struct DragonDepot {
    pub collected_item1: String,
    pub collected_item2: String,
    item_count: usize,
    burnstone_signal: Arc<(Mutex<bool>, Condvar)>,
    seaplum_signal: Arc<(Mutex<bool>, Condvar)>,
    klah_signal: Arc<(Mutex<bool>, Condvar)>
}

impl DragonDepot {
    /// Constructs a new `Dragon Depot` instance with the ability to place
    /// and deplete resources. It also has the ability to check to see if it
    /// currently has any of the resources.
    pub fn new(burnstone_signal:Arc<(Mutex<bool>, Condvar)>,
               seaplum_signal:Arc<(Mutex<bool>, Condvar)>,
               klah_signal:Arc<(Mutex<bool>, Condvar)>) -> DragonDepot {
        DragonDepot {
            collected_item1: String::new(),
            collected_item2: String::new(),
            item_count: 0,
            burnstone_signal: burnstone_signal,
            seaplum_signal: seaplum_signal,
            klah_signal: klah_signal
        }
    }

    /// Places a resource in the depot and checks
    /// if it is time to notify the respective stronghold.
    ///
    /// # Parameters
    /// - `resource`: The type of resource being placed into the depot.
    ///
    /// This method increments the item count and places the resource into an available slot.
    /// Once two resources are collected, it checks each resource type and signals the
    /// corresponding stronghold if its resource is available. After signaling,
    /// it depletes the resources.
    pub fn place_resource(&mut self, resource:String) {
        if self.item_count == 0 {
            self.collected_item1 = resource;
        } else {
            self.collected_item2 = resource;
        }
        self.item_count += 1;
        if self.item_count == MAX_ITEM {
            if !self.has_burnstone() {
                let (lock, condvar) = &*self.burnstone_signal;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            } else if !self.has_seaplum() {
                let (lock, condvar) = &*self.seaplum_signal;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            } else if !self.has_klah() {
                let (lock, condvar) = &*self.klah_signal;
                let mut ready = lock.lock().unwrap();
                *ready = true;
                condvar.notify_one();
            } else {
                unreachable!()
            }
            self.deplete();
        }
    }

    /// Test to see if klah is one of the collected items
    fn has_klah(&self) -> bool {
        self.collected_item1 == String::from("Klah") ||
        self.collected_item2 == String::from("Klah")
    }

    /// Test to see if burnstone is one of the collected items
    fn has_burnstone(&self) -> bool {
        self.collected_item1 == String::from("Burnstone") ||
        self.collected_item2 == String::from("Burnstone")
    }

    /// Test to see if seaplum is one of the collected items
    fn has_seaplum(&self) -> bool {
        self.collected_item1 == String::from("Seaplum") ||
        self.collected_item2 == String::from("Seaplum")
    }

    /// Sets the item_count to 0 and makes the
    /// collected items reset to fresh strings.
    fn deplete(&mut self) {
        self.item_count = 0;
        self.collected_item1 = String::new();
        self.collected_item2 = String::new();
    }

}