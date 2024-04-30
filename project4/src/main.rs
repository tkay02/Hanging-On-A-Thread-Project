pub mod steward;
pub mod dragonrider;
pub mod stronghold;
mod depot;

use std::sync::{Arc, Mutex};
use std::thread;

use depot::Depot;
use steward::Steward;

fn main() {
    //Testing stuff
    let depot = Arc::new(Mutex::new(Depot::new()));
    println!("Depot has been created");
    let mut steward = Steward::new(Arc::clone(&depot));
    println!("Steward has been created");

    let thread_handler = thread::spawn(move || {
        println!("Steward will start producing");
        steward.produce();
        println!("Steward has finished producing");
    });

    let _joined = thread_handler.join();

    let lock = &*depot;
    let storage = lock.lock().unwrap();
    println!("Test run results: \n{}", storage.test_status());

    
}
