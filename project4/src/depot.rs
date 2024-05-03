//! # Hanging on by a Thread: depot.rs
//! # Version: May 3rd 2024
//!
//! ## Description
//! Sets up the depot for holding each type of resource:
//! - klah
//! - burnstone
//! - seaplum
//! Has the functionality of taking or placing each resource, checking if it is empty,
//! and testing the status of each resource.
//!
//! ## Authors
//! - Dylan Miller
//! - Thomas Kay
//!
//! ## Instructor
//! - Dr. William Kreahling

use crate::steward::RESOURCES;

/// Represents a Depot that houses resources
///
/// # Fields
/// - `burnstone`: the length of each needle used in the experiment.
/// - `seaplum`: the distance between parallel lines on the surface.
/// - `klah`: total number of needles to be dropped in the experiment.
pub struct Depot {
    burnstone: String,
    seaplum: String,
    klah: String
}

impl Depot {
    
    pub fn new() -> Depot {
        Depot {
            burnstone: String::new(),
            seaplum: String::new(),
            klah: String::new()
        }
    }

    pub fn place_burnstone(&mut self) {
        self.burnstone = String::from(RESOURCES[0]);
    }

    pub fn take_burnstone(&mut self) -> String {
        let burnstone = self.burnstone.clone();
        self.burnstone = String::new();
        burnstone
    }

    pub fn place_seaplum(&mut self) {
        self.seaplum = String::from(RESOURCES[1]);
    }

    pub fn take_seaplum(&mut self) -> String {
        let seaplum = self.seaplum.clone();
        self.seaplum = String::new();
        seaplum
    }

    pub fn place_klah(&mut self) {
        self.klah = String::from(RESOURCES[2]);
    }

    pub fn take_klah(&mut self) -> String {
        let klah = self.klah.clone();
        self.klah = String::new();
        klah
    }

    pub fn test_status(&self) -> String {
        let mut result1 = "false";
        if self.burnstone != String::new() { result1 = "true"; }
        let mut result2 = "false";
        if self.seaplum != String::new() { result2 = "true"; }
        let mut result3 = "false";
        if self.klah != String::new() { result3 = "true"; }
        String::new().to_owned() + "Burnstone obtained: " + result1 +
        "\nSeaplum obtained: " + result2 + "\nKlah obtained: " + result3
    }

    pub fn is_empty(&self) -> bool {
        self.burnstone == String::new() && self.seaplum == String::new() &&
        self.klah == String::new()
    }

}