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

    



}