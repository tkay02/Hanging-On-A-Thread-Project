use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};

use rand::{thread_rng, Rng};

const MIN_SECONDS:f64 = 5.0;

//Need a reference to a mutex
pub struct Stronghold {
    // The name of the stronghold (which contains the resource)
    name: String,
    // Signal to tell steward that supplies have been successfully received
    resources_received: Arc<(Mutex<bool>, Condvar)>,
    // Signal to receive that the resources that the stronghold is lacking is available
    resources_available: Arc<(Mutex<bool>, Condvar)>,
    // Signal to tell dragon riders to deliever needed resources
    resources_deliever: Arc<(Mutex<bool>, Condvar)>
}

impl Stronghold {

    pub fn new(name: String,
               resources_received: Arc<(Mutex<bool>, Condvar)>,
               resources_available: Arc<(Mutex<bool>, Condvar)>,
               resources_deliever: Arc<(Mutex<bool>, Condvar)>) -> Stronghold {
        Stronghold {
            name: name,
            resources_received: resources_received,
            resources_available: resources_available,
            resources_deliever: resources_deliever
        }
    }

    fn wait_for_resources(&self) {
        
    }

    fn distribute_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now distributing resources", self.name);
        thread::sleep(time);
        println!("Stronghold {} has finished distributing resources", self.name);
    }

    fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now consuming resources", self.name);
        thread::sleep(time);
        println!("Stronghold {} has finished consuming resources", self.name);
    }

}