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
/// - `burnstone`: provides defense and power to the strongholds
/// - `seaplum`:  a delicious and nutritious foodstuff
/// - `klah`:  a nutritious and tasty drink
pub struct Depot {
    burnstone: String,
    seaplum: String,
    klah: String
}

impl Depot {
    /// Constructs a new `Depot` instance with the ability to place,
    /// take, test the status, and see if the depot is empty.
    pub fn new() -> Depot {
        Depot {
            burnstone: String::new(),
            seaplum: String::new(),
            klah: String::new()
        }
    }

    /// Places burnstone within the depot itself.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn place_burnstone(&mut self) {
        self.burnstone = String::from(RESOURCES[0]);
    }

    /// Takes a new cloned instance of burnstone.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn take_burnstone(&mut self) -> String {
        let burnstone = self.burnstone.clone();
        self.burnstone = String::new();
        burnstone
    }

    /// Places seaplum within the depot itself.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn place_seaplum(&mut self) {
        self.seaplum = String::from(RESOURCES[1]);
    }

    /// Takes a new cloned instance of seaplum.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn take_seaplum(&mut self) -> String {
        let seaplum = self.seaplum.clone();
        self.seaplum = String::new();
        seaplum
    }

    /// Places klah within the depot itself.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn place_klah(&mut self) {
        self.klah = String::from(RESOURCES[2]);
    }

    /// Takes a new cloned instance of klah.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn take_klah(&mut self) -> String {
        let klah = self.klah.clone();
        self.klah = String::new();
        klah
    }

    /// Tests the status of all the resources to see if
    /// they have been obtained by the depot or not.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
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

    /// Test to see if the depot itself is empty.
    ///
    /// # Parameters
    /// - `self`: A reference to the depot itself.
    pub fn is_empty(&self) -> bool {
        self.burnstone == String::new() && self.seaplum == String::new() &&
        self.klah == String::new()
    }

}