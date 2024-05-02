//! # Hanging on by a Thread: dragondepot.rs
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

use std::sync::{Arc,Mutex,Condvar};



const MAX_ITEM:usize = 2;

pub struct DragonDepot {
    // First item collected by the dragon riders
    pub collected_item1: String,
    // Second item collected by the dragon riders
    pub collected_item2: String, 
    // The amount of items stored in the dragon depot
    item_count: usize,
    // Signal for burnstone stronghold that its resources are available
    burnstone_signal: Arc<(Mutex<bool>, Condvar)>,
    // Signal for seaplum stronghold that its resources are available
    seaplum_signal: Arc<(Mutex<bool>, Condvar)>,
    // Signal for klah stronghold that its resources are available
    klah_signal: Arc<(Mutex<bool>, Condvar)>
}

impl DragonDepot {
    
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

    pub fn place_resource(&mut self, resource:String) {
        if self.item_count == 0 {
            self.collected_item1 = resource;
        } else {
            self.collected_item2 = resource;
        }
        self.item_count += 1;
        // if self.item_count == MAX_ITEM {
        //     if !self.has_burnstone() {
        //         let (lock, condvar) = &*self.burnstone_signal;
        //         let mut ready = lock.lock().unwrap();
        //         *ready = true;
        //         condvar.notify_one();
        //     } else if !self.has_seaplum() {
        //         let (lock, condvar) = &*self.seaplum_signal;
        //         let mut ready = lock.lock().unwrap();
        //         *ready = true;
        //         condvar.notify_one();
        //     } else if !self.has_klah() {
        //         let (lock, condvar) = &*self.klah_signal;
        //         let mut ready = lock.lock().unwrap();
        //         *ready = true;
        //         condvar.notify_one();
        //     } else {
        //         unreachable!()
        //     }
        //     self.deplete();
        // }
    }

    fn has_klah(&self) -> bool {
        self.collected_item1 == String::from("Klah") ||
        self.collected_item2 == String::from("Klah")
    }

    fn has_burnstone(&self) -> bool {
        self.collected_item1 == String::from("Burnstone") ||
        self.collected_item2 == String::from("Burnstone")
    }

    fn has_seaplum(&self) -> bool {
        self.collected_item1 == String::from("Seaplum") ||
        self.collected_item2 == String::from("Seaplum")
    }

    fn deplete(&mut self) {
        self.item_count = 0;
        self.collected_item1 = String::new();
        self.collected_item2 = String::new();
    }

}