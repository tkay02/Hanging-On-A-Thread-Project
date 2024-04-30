use std::{thread, time::Duration};

use rand::{thread_rng, Rng};

const MIN_SECONDS:f64 = 5.0;

pub struct Stronghold {
    main_resource: String
}

impl Stronghold {

    pub fn distribute_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now distributing resources", self.main_resource);
        thread::sleep(time);
        println!("Stronghold {} has finished distributing resources", self.main_resource);
    }

    pub fn consume_resources(&self) {
        let mut rng = thread_rng();
        let time_rng = ((rng.gen::<f64>() * MIN_SECONDS) + MIN_SECONDS) as u64;
        let time = Duration::from_secs(time_rng);
        println!("Stronghold {} is now consuming resources", self.main_resource);
        thread::sleep(time);
        println!("Stronghold {} has finished consuming resources", self.main_resource);
    }

}