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
//! This module depends on the following external crate:
//! - use std::sync::{Arc,Mutex,Condvar};
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use std::sync::{Arc, Condvar, Mutex};

use crate::{depot::Depot, dragondepot::DragonDepot};


pub struct DragonRider {
    // Type of resource that the dragonrider is carrying
    resource_type: String,
    // Reference to depot to obtain resources
    depot: Arc<Mutex<Depot>>,
    // Reference to depot to store resources obtained by the dragonriders
    dragon_depot: Arc<Mutex<DragonDepot>>,
    // Signal that depot has resource that is ready to be collected
    depot_signal: Arc<(Mutex<bool>, Condvar)>,
}

impl DragonRider {

    pub fn new(resource:String, 
               depot:Arc<Mutex<Depot>>,
               dragon_depot:Arc<Mutex<DragonDepot>>,
               depot_signal:Arc<(Mutex<bool>, Condvar)>) -> DragonRider {
        DragonRider {
            resource_type: resource,
            depot: depot,
            dragon_depot: dragon_depot,
            depot_signal: depot_signal
        }
    }

    pub fn waiting_for_resource(&self) -> String {
        self.resource_type.clone() + " dragon rider is waiting for resource"
    }

    pub fn obtained_resource(&self) -> String {
        self.resource_type.clone() + " dragon rider has obtained resource"
    }

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
                let _ = &depot.take_kleh();
            }
            _ => { unreachable!() }
        }
        println!("{}", self.obtained_resource());
    }

    pub fn wait_for_consumation(&self) {
        let (lock, condvar) = &*self.depot_signal;
        let guard = lock.lock().unwrap();
        println!("{}",self.waiting_for_resource());
        let mut guard = condvar.wait_while(guard, |condition| {
            !*condition
        }).unwrap();
        *guard = false;
    }

    pub fn group_resources(&mut self) {
        let lock = &*self.dragon_depot;
        let mut dragon_depot = lock.lock().unwrap();
        dragon_depot.place_resource(self.resource_type.clone());
    }

    pub fn go(&mut self) {
        self.wait_for_consumation();
        self.consume();
        self.group_resources();
    }

}