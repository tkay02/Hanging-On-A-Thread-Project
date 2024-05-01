use std::sync::{Arc, Condvar, Mutex};

use crate::depot::Depot;


pub struct DragonRider {
    resource_type: String,
    depot: Arc<Mutex<Depot>>,
    depot_signal: Arc<(Mutex<bool>, Condvar)>,
    deliever_signal: Arc<(Mutex<bool>, Condvar)>
}

impl DragonRider {

    pub fn new(resource:String, 
               depot:Arc<Mutex<Depot>>, 
               depot_signal:Arc<(Mutex<bool>, Condvar)>,
               deliever_signal:Arc<(Mutex<bool>, Condvar)>) -> DragonRider {
        DragonRider {
            resource_type: resource,
            depot: depot,
            depot_signal: depot_signal,
            deliever_signal: deliever_signal
        }
    }

    fn consume(&self) {
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        match self.resource_type.as_str() {
            "Burnstone" => {
                &depot.take_burnstone();
            },
            "Seaplum" => {
                &depot.take_seaplum();
            },
            "Klah" => {
                &depot.take_kleh();
            }
            _ => { unreachable!() }
        }
    }

    fn wait_for_consumation(&self) {
        let (lock, condvar) = &*self.depot_signal;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
            println!("Waiting for received condition");
            !*condition
        }).unwrap();
        *guard = false;
    }

    fn wait_for_delievery(&self) {
        let (lock, condvar) = &*self.deliever_signal;
        let mut guard = condvar.wait_while(lock.lock().unwrap(), |condition| {
            println!("Waiting for delievery condition");
            !*condition
        }).unwrap();
        *guard = false;
    }

    pub fn go(&self) {
        self.wait_for_consumation();
        self.consume();
        self.wait_for_delievery();
    }

}