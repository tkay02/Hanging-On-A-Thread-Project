use std::sync::Condvar;
use rand::rngs::SmallRng;
use rand::{thread_rng, Rng};

const MAX_RESOURCES:f64 = 3.0;

pub const RESOURCES:[&'static str; 3] = ["Burnstone", "Seaplum", "Klah"];

/// Input fields for steward
pub struct Steward {
    
}