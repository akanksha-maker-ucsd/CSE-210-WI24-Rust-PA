use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    beaches: Vec<Beach>,
    reefs: Vec<Rc<RefCell<Reef>>>
}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean {
            beaches: Vec::new(),
            reefs: Vec::new()
        }
    }

    pub fn add_beach(&mut self, beach: Beach) {
       self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let mut reef = Reef::new();
        for _ in 0..n_minnows {
            reef.add_prey(Box::new(Minnow::new(25)));
        }
        for _ in 0..n_shrimp {
            reef.add_prey(Box::new(Shrimp::new(1)));
        }
        for _ in 0..n_clams {
            reef.add_prey(Box::new(Clam::new()));
        }
        for _ in 0..n_algae {
            reef.add_prey(Box::new(Algae::new()));
        }
        let reef_rc = Rc::new(RefCell::new(reef));
        self.reefs.push(reef_rc.clone());
        reef_rc

    }
}
