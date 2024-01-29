use std::collections::HashMap;
use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use std::slice::Iter;

#[derive(Debug)]
pub struct ClanSystem {
    clans: HashMap<String, HashMap<String, Crab>>
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {
            clans: HashMap::new()
        }
    }

    pub fn create_clan(&mut self, clan_id: String) {
        self.clans.insert(clan_id, HashMap::new());
    }
    
    pub fn add_member(&mut self, clan_id: &str, member_name: String, crab: Crab) {
        self.clans.get_mut(clan_id).unwrap().insert(member_name, crab);
    }

    pub fn get_clan(&mut self, clan_id: &str) -> Option<&HashMap<String, Crab>> {
        self.clans.get(clan_id)
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if let Some(clan) = self.clans.get(clan_id) {
            clan.keys().cloned().collect()
        } else {
            Vec::new() // Return an empty vector if the clan doesn't exist
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        return self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        if let Some(clan) = self.clans.get(clan_id) {
            clan.len()
        } else {
            0 // Return 0 if the clan doesn't exist
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest_clan_id = None;
        let mut largest_clan_size = 0;
        for (clan_id, clan) in &self.clans {
            let clan_size = self.get_clan_member_count(clan_id);
            if clan_size > largest_clan_size {
                largest_clan_size = clan_size;
                largest_clan_id = Some(clan_id.clone());
            }
        
        
    }

    return largest_clan_id;
}

}
