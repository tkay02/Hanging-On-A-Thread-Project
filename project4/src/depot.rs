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

/// Structure that represents a Depot that houses resources
///
/// # Fields
/// - `burnstone`: provides defense and power to the strongholds
/// - `seaplum`:  a delicious and nutritious foodstuff
/// - `klah`:  a nutritious and tasty drink
pub struct Depot {
    burnstone: String,
    seaplum: String,
    klah: String
}

impl Depot {
    /// Creates a new, empty `Depot`.
    pub fn new() -> Depot {
        Depot {
            burnstone: String::new(),
            seaplum: String::new(),
            klah: String::new()
        }
    }

    /// Places burnstone within the depot.
    pub fn place_burnstone(&mut self) {
        self.burnstone = String::from(RESOURCES[0]);
    }

    /// Retrieves all burnstone from the depot, leaving it empty.
    ///
    /// # Returns
    /// A `String` containing the burnstone that was stored.
    pub fn take_burnstone(&mut self) -> String {
        let burnstone = self.burnstone.clone();
        self.burnstone = String::new();
        burnstone
    }

    /// Places seaplum within the depot.
    pub fn place_seaplum(&mut self) {
        self.seaplum = String::from(RESOURCES[1]);
    }

    /// Retrieves all seaplum from the depot, leaving it empty.
    ///
    /// # Returns
    /// A `String` containing the seaplum that was stored.
    pub fn take_seaplum(&mut self) -> String {
        let seaplum = self.seaplum.clone();
        self.seaplum = String::new();
        seaplum
    }

    /// Places klah within the depot.
    pub fn place_klah(&mut self) {
        self.klah = String::from(RESOURCES[2]);
    }

    /// Retrieves all klah from the depot, leaving it empty.
    ///
    /// # Returns
    /// A `String` containing the klah that was stored.
    pub fn take_klah(&mut self) -> String {
        let klah = self.klah.clone();
        self.klah = String::new();
        klah
    }

}