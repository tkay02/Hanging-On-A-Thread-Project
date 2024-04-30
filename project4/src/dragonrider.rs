use std::sync::{Arc, Condvar, Mutex};

use crate::depot::Depot;



pub struct DragonRider {
    main_resource: String,
    depot: Arc<Mutex<Depot>>,
    alert: Condvar
}

impl DragonRider {

    pub fn new(resource:String, depot:Arc<Mutex<Depot>>) -> DragonRider {
        DragonRider {
            main_resource: resource,
            depot: depot,
            alert: Condvar::new()
        }
    }

    pub fn consume(&mut self) {
        let lock = &*self.depot;
        let mut depot = lock.lock().unwrap();
        match self.main_resource.as_str() {
            "Burnstone" => {
                let _ = &depot.take_burnstone();
            },
            "Seaplum" => {
                let _ = &depot.take_seaplum();
            },
            _ => {
                let _ = &depot.take_kleh();
            }
        }
    }

    





}