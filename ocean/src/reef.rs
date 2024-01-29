use crate::prey::Prey;

// VecDeque is Rust's implementation of a double-ended queue, and
// is used only if we only need to use it in a single-ended manner.
use std::collections::vec_deque::{Iter, VecDeque};

#[derive(Debug)]
pub struct Reef {
    prey: VecDeque<Box<dyn Prey>>
}

impl Reef {
    /**
     * Creates a new reef.
     */
    pub fn new() -> Self {
        Reef{
            prey: VecDeque::new()
        }
    }

    pub fn prey(&self) -> Iter<Box<dyn Prey>> {
        return self.prey.iter(); 
    }

    pub fn population(&self) -> usize {
        return self.prey.len();
    }

    /**
     * Adds a prey to the reef.
     *
     * This function takes ownership of the boxed prey.
     */
    pub fn add_prey(&mut self, prey: Box<dyn Prey>) {
       self.prey.push_back(prey);
    }

    /**
     * Returns the next available prey.
     *
     * The callee of this function receives ownership of the boxed prey.
     */
    pub fn take_prey(&mut self) -> Option<Box<dyn Prey>> {
        if self.prey.is_empty() {
            None
        } else {
            Some(self.prey.remove(0)?)
        }
    }
}
