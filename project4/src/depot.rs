use crate::steward::RESOURCES;

pub struct Depot {
    burnstone: String,
    seaplum: String,
    kleh: String
}

impl Depot {
    
    pub fn new() -> Depot {
        Depot {
            burnstone: String::new(),
            seaplum: String::new(),
            kleh: String::new()
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

    pub fn place_kleh(&mut self) {
        self.kleh = String::from(RESOURCES[2]);
    }

    pub fn take_kleh(&mut self) -> String {
        let kleh = self.kleh.clone();
        self.kleh = String::new();
        kleh
    }

    pub fn test_status(&self) -> String {
        let mut result1 = "false";
        if self.burnstone != String::new() { result1 = "true"; }
        let mut result2 = "false";
        if self.seaplum != String::new() { result2 = "true"; }
        let mut result3 = "false";
        if self.kleh != String::new() { result3 = "true"; }
        String::new().to_owned() + "Burnstone obtained: " + result1 +
        "\nSeaplum obtained: " + result2 + "\nKleh obtained: " + result3
    }

}