pub mod steward;
pub mod dragonrider;
pub mod stronghold;
mod depot;
pub mod dragondepot;
mod stronghold_supply;

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use depot::Depot;
use steward::Steward;

use dragonrider::DragonRider;

use crate::dragondepot::DragonDepot;

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

    let dragon_depot = Arc::new(Mutex::new(DragonDepot::new(
        Arc::clone(&burnstone_deliever_signal), Arc::clone(&seaplum_deliever_signal),
         Arc::clone(&klah_deliever_signal))));


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
        Arc::clone(&dragon_depot),
        Arc::clone(&burnstone_get_signal), 
        Arc::clone(&burnstone_deliever_signal)
    );

    let mut dragonrider2:DragonRider = DragonRider::new(
        seaplum, 
        Arc::clone(&depot), 
        Arc::clone(&dragon_depot),
        Arc::clone(&seaplum_get_signal), 
        Arc::clone(&seaplum_deliever_signal)
    );

    let mut dragonrider3:DragonRider = DragonRider::new(
        klah, 
        Arc::clone(&depot), 
        Arc::clone(&dragon_depot),
        Arc::clone(&klah_get_signal), 
        Arc::clone(&klah_deliever_signal)
    );
    

    let _handler1 = thread::spawn(move || {
        steward.produce();
    });

    let _handler2 = thread::spawn(move || {
        dragonrider1.wait_for_consumation();
        dragonrider1.consume();
        dragonrider1.group_resources();
    });

    let _handler3 = thread::spawn(move || {
        dragonrider2.wait_for_consumation();
        dragonrider2.consume();
        dragonrider2.group_resources();
    });

    let _handler4 = thread::spawn(move || {
        dragonrider3.wait_for_consumation();
        dragonrider3.consume();
        dragonrider3.group_resources();
    });

    
    thread::sleep(Duration::from_secs(5));
    let lock = &*dragon_depot;
    let storage = lock.lock().unwrap();
    println!("Resource1: {} \nResource2: {}", storage.collected_item1, storage.collected_item2);
    
}
