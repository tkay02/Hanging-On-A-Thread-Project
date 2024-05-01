pub mod steward;
pub mod dragonrider;
pub mod stronghold;
mod depot;

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use depot::Depot;
use steward::Steward;

use dragonrider::DragonRider;

fn main() {
    //Testing stuff
    let depot = Arc::new(Mutex::new(Depot::new()));
    let burnstone_get_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let seaplum_get_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let klah_get_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let refill_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let burnstone_deliever_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let seaplum_deliever_signal = Arc::new((Mutex::new(false), Condvar::new()));
    let klah_deliever_signal = Arc::new((Mutex::new(false), Condvar::new()));

    let mut steward = Steward::new(
        Arc::clone(&depot),
        Arc::clone(&refill_signal),
        Arc::clone(&burnstone_get_signal),
        Arc::clone(&seaplum_get_signal),
        Arc::clone(&klah_get_signal)
    );

    let burnstone = String::from("Burnstone");
    let seaplum = String::from("Seaplum");
    let klah = String::from("Klah");

    let mut dragonrider1:DragonRider = DragonRider::new(
        burnstone, 
        Arc::clone(&depot), 
        Arc::clone(&burnstone_get_signal), 
        Arc::clone(&burnstone_deliever_signal)
    );

    let mut dragonrider2:DragonRider = DragonRider::new(
        seaplum, 
        Arc::clone(&depot), 
        Arc::clone(&seaplum_get_signal), 
        Arc::clone(&seaplum_deliever_signal)
    );

    let mut dragonrider3:DragonRider = DragonRider::new(
        klah, 
        Arc::clone(&depot), 
        Arc::clone(&klah_get_signal), 
        Arc::clone(&klah_deliever_signal)
    );
    

    let _handler1 = thread::spawn(move || {
        steward.produce();
    });

    let _handler2 = thread::spawn(move || {
        dragonrider1.wait_for_consumation();
        dragonrider1.consume();
    });

    let _handler3 = thread::spawn(move || {
        dragonrider2.wait_for_consumation();
        dragonrider2.consume();
    });

    let _handler4 = thread::spawn(move || {
        dragonrider3.wait_for_consumation();
        dragonrider3.consume();
    });

    
    thread::sleep(Duration::from_secs(5));
    let lock = &*depot;
    let storage = lock.lock().unwrap();
    println!("{}", storage.test_status());
    
}
