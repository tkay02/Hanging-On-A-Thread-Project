use std::sync::{Arc, Condvar, Mutex};
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
    klah_ready: Arc<(Mutex<bool>, Condvar)>
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
            klah_ready: klah
        }
    }

    fn collect_resources() -> (String, String) {
        let mut rng = thread_rng();
        let rng1 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        let mut rng2 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        while rng1 == rng2 {
            rng2 = (rng.gen::<f64>() * MAX_RESOURCES) as usize;
        }
        let resource1 = String::from(RESOURCES[rng1]);
        let resource2 = String::from(RESOURCES[rng2]);
        (resource1, resource2)
    }

    pub fn produce(&mut self) {
        let (resource1, resource2) = Steward::collect_resources();
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        self.notify_resource(resource1, &depot);
        self.notify_resource(resource2, &depot);
    }

    fn notify_resource(&mut self, resource:String, depot:&Depot) {
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

    fn wait_for_received(&self) {
        let (lock, condvar) = &*self.stronghold_received;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
            println!("Waiting for received condition");
            !*condition
        }).unwrap();
        *guard = false;
    }

    pub fn go(&mut self) {
        self.produce();
        self.wait_for_received();
    }


}