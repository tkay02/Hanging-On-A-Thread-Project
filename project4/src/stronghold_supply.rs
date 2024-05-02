use std::sync::{Condvar, Arc, Mutex};



const MAX_COUNT:usize = 2;

pub struct StrongholdSupply {
    item_count: usize,
    stronghold_signal: Arc<(Mutex<bool>, Condvar)>
}

impl StrongholdSupply {

    pub fn new(stronghold_signal:Arc<(Mutex<bool>, Condvar)>) -> StrongholdSupply {
        StrongholdSupply {
            item_count: 0,
            stronghold_signal: stronghold_signal
        }
    }
    
    pub fn deliever_resource(&mut self) {
        self.item_count += 1;
        if self.item_count == MAX_COUNT {
            let (lock, condvar) = &*self.stronghold_signal;
            let mut signal = lock.lock().unwrap();
            *signal = true;
            condvar.notify_one();
        }
    }
}