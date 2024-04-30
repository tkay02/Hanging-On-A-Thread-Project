use std::{sync::Condvar, thread, time::Duration};

use rand::{thread_rng, Rng};

const MIN_SECONDS:f64 = 5.0;

//Need a reference to a mutex
pub struct Stronghold {
    main_resource: String,
    signal: Condvar
}

impl Stronghold {

    pub fn new(resource:String) -> Stronghold {
        Stronghold {
            main_resource: resource,
            signal: Condvar::new()
        }
    }

    fn distribute_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now distributing resources", self.main_resource);
        thread::sleep(time);
        println!("Stronghold {} has finished distributing resources", self.main_resource);
    }

    fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now consuming resources", self.main_resource);
        thread::sleep(time);
        println!("Stronghold {} has finished consuming resources", self.main_resource);
    }

    pub fn obtain_resources(&self) {
        self.signal.notify_one();
        self.distribute_resources();
        self.consume_resources();
    }

}