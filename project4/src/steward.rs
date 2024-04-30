use std::sync::{Arc, Condvar, Mutex};
use rand::{thread_rng, Rng};

use crate::depot::Depot;

const MAX_RESOURCES:f64 = 3.0;

pub const RESOURCES:[&'static str; 3] = ["Burnstone", "Seaplum", "Klah"];

/// Input fields for steward
pub struct Steward {
    depot: Arc<Mutex<Depot>>,
    burnstone_cond: Condvar,
    seaplum_cond: Condvar,
    kleh_cond: Condvar,
    resource1: String,
    resource2: String
}

impl Steward {
    pub fn new(depot:Arc<Mutex<Depot>>) -> Steward {
        Steward {
            depot: depot,
            burnstone_cond: Condvar::new(),
            seaplum_cond: Condvar::new(),
            kleh_cond: Condvar::new(),
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

    fn place_resources(&self) {
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        match self.resource1.as_str() {
            "Burnstone" => {
                depot.place_burnstone();
                self.burnstone_cond.notify_one();
            },
            "Seaplum" => {
                depot.place_seaplum();
                self.seaplum_cond.notify_one();
            }
            _ => {
                depot.place_kleh();
                self.kleh_cond.notify_one();
            }
        }
        match self.resource2.as_str() {
            "Burnstone" => {
                depot.place_burnstone();
                self.burnstone_cond.notify_one();
            },
            "Seaplum" => {
                depot.place_seaplum();
                self.seaplum_cond.notify_one();
            }
            _ => {
                depot.place_kleh();
                self.kleh_cond.notify_one();
            }
        }
    }

    pub fn produce(&mut self) {
        self.collect_resources();
        self.place_resources();
    }

    pub fn wait(&self) {

    }

}