use std::{cell::RefCell, collections::HashSet, rc::Rc};
use rand::rngs::SmallRng;
use rand::Rng;

pub struct RobotFactory {
    registry: Rc<RefCell<HashSet<String>>>,
}

#[derive(Clone)]
pub struct Robot {
    name: String,
    registry: Rc<RefCell<HashSet<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            registry: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot(&mut self, rng: &mut SmallRng) -> Robot {
        let registry = Rc::clone(&self.registry);

        let name = loop {
            let letter1 = (b'A' + (rng.next_u32() % 26) as u8) as char;
            let letter2 = (b'A' + (rng.next_u32() % 26) as u8) as char;
            let num = (rng.next_u32() % 1000) as u32;

            let candidate = format!("{}{}{:03}", letter1, letter2, num);

            if !registry.borrow().contains(&candidate) {
                registry.borrow_mut().insert(candidate.clone());
                break candidate;
            }
        };

        Robot { name, registry }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset(&mut self, rng: &mut SmallRng) {
        // Free old name so it becomes available again
        let old = self.name.clone();
        self.registry.borrow_mut().remove(&old);

        let new_name = loop {
            let letter1 = (b'A' + (rng.next_u32() % 26) as u8) as char;
            let letter2 = (b'A' + (rng.next_u32() % 26) as u8) as char;
            let num = (rng.next_u32() % 1000) as u32;

            let candidate = format!("{}{}{:03}", letter1, letter2, num);

            if !self.registry.borrow().contains(&candidate) {
                self.registry.borrow_mut().insert(candidate.clone());
                break candidate;
            }
        };

        self.name = new_name;
    }
}