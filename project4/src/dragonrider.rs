//! # Hanging on by a Thread: dragonrider.rs
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
    // Signal to deliever resource to a stronghold
    deliever_signal: Arc<(Mutex<bool>, Condvar)>
}

impl DragonRider {

    pub fn new(resource:String, 
               depot:Arc<Mutex<Depot>>,
               dragon_depot:Arc<Mutex<DragonDepot>>,
               depot_signal:Arc<(Mutex<bool>, Condvar)>,
               deliever_signal:Arc<(Mutex<bool>, Condvar)>) -> DragonRider {
        DragonRider {
            resource_type: resource,
            depot: depot,
            dragon_depot: dragon_depot,
            depot_signal: depot_signal,
            deliever_signal: deliever_signal
        }
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
    }

    pub fn wait_for_consumation(&self) {
        let (lock, condvar) = &*self.depot_signal;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
            println!("Waiting for received condition");
            !*condition
        }).unwrap();
        *guard = false;
    }

    fn wait_for_delievery(&self) {
        let (lock, condvar) = &*self.deliever_signal;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
            println!("Waiting for delievery condition");
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
        self.wait_for_delievery();
    }

}