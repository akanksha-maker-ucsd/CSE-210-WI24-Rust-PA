use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    crabs: Vec<Crab>,
    clan_system: ClanSystem
}

impl Beach {
    pub fn new() -> Beach {
        Beach {
            crabs: Vec::new(),
            clan_system: ClanSystem::new()
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        return self.crabs.len(); 
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        return &self.crabs[index];
    }

    pub fn crabs(&self) -> Iter<Crab> {
        return self.crabs.iter();
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        return self.crabs.iter().max_by_key(|crab| crab.speed());

    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crabs_with_name: Vec<&Crab> = Vec::new();
        for crab in self.crabs.iter() {
            if crab.name() == name {
                crabs_with_name.push(crab);
            }
        }
        return crabs_with_name;
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        
        if i>= self.crabs.len() || j>= self.crabs.len() {
            panic!("Index out of bounds");
        }
        
        let crab1 = &self.crabs[i];
        let crab2 = &self.crabs[j];
        let mut new_crab = Crab::breed(name,crab1, crab2); 
        self.crabs.push(new_crab);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {

        let crab = self.crabs.iter().find(|crab| crab.name() == crab_name);
       // Check if the clan exists, if not, create a new clan
       if !self.clan_system.get_clan(clan_id).is_some() {
            self.clan_system.create_clan(clan_id.to_string());
        }

        // Add crab to the clan
        self.clan_system.add_member(clan_id, crab_name.to_string(), crab.unwrap().clone());
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
     pub fn get_winner_clan(&mut self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1 = self.clan_system.get_clan(id1).cloned();
        let clan2 = self.clan_system.get_clan(id2).cloned();
        
        if clan1.is_none() || clan2.is_none() {
            return Err("Clan does not exist".to_string());
        }


        let mut total_clan1_speed = 0; 
        let mut total_clan2_speed = 0;  
        
        for crab in clan1.unwrap().values() {
            total_clan1_speed += crab.speed();
        }
        for crab in clan2.unwrap().values() {
            total_clan2_speed += crab.speed();
        }

        let clan1_count = self.clan_system.get_clan_member_count(id1) as u32;
        let clan2_count = self.clan_system.get_clan_member_count(id2) as u32;
        let avg_clan1_speed = total_clan1_speed / clan1_count;
        let avg_clan2_speed = total_clan2_speed / clan2_count;

        if avg_clan1_speed > avg_clan2_speed {
            Ok(Some(id1.to_string()))
        } else if avg_clan2_speed > avg_clan1_speed {
            Ok(Some(id2.to_string()))
        } else {
            // If the average speeds are equal, there's no clear winner
            Ok(None)
        }
    }
    
}
