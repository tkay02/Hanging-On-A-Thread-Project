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
    resources_deliever1: Arc<(Mutex<bool>, Condvar)>,
    resources_deliever2: Arc<(Mutex<bool>, Condvar)>
}

impl Stronghold {

    pub fn new(name: String,
               resources_received: Arc<(Mutex<bool>, Condvar)>,
               resources_available: Arc<(Mutex<bool>, Condvar)>,
               resources_deliever1: Arc<(Mutex<bool>, Condvar)>,
               resources_deliever2: Arc<(Mutex<bool>, Condvar)>) -> Stronghold {
        Stronghold {
            name: name,
            resources_received: resources_received,
            resources_available: resources_available,
            resources_deliever1: resources_deliever1,
            resources_deliever2: resources_deliever2
        }
    }

    fn wait_for_resources(&self) {
        let (lock, condvar) = &*self.resources_available;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
           println!("Stronghold is waiting for resources");
           !*condition 
        }).unwrap();
        *guard = false;
    }

    fn deliever_my_resources(&self) {
        
    }

    
    fn resources_received(&self) {
        let (lock, condvar) = &*self.resources_received;
        let mut received = lock.lock().unwrap();
        *received = true;
        condvar.notify_one();
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

    pub fn go(&mut self) {
        self.wait_for_resources();
        self.resources_received();
        self.distribute_resources();
        self.consume_resources();
    }

}