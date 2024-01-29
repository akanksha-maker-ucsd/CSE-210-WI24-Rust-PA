extern crate ocean;

/*
 * Run `cargo test public` to run all of the provided tests.
 */

use ocean::beach::*;
use ocean::color::*;
use ocean::cookbook::*;
use ocean::crab::*;
use ocean::diet::*;
use ocean::ocean::*;
use ocean::prey::*;
use ocean::reef::*;

use std::cell::RefCell;
use std::rc::Rc;

/* Fixtures */

static PRINZ: &str = "Prinz";
static SEBASTIAN: &str = "Sebastian";
static CRAB_1: &str = "Edward";
static CRAB_2: &str = "Mira";
static CRAB_3: &str = "Hermione";
static CRAB_4: &str = "Luna";
static CRAB_5: &str = "Cancer";
static CLAN_1: &str = "Pincher Patrol";
static CLAN_2: &str = "Reef Raiders";
static CLAN_3: &str = "Shell Shockers";

// https://en.wikipedia.org/wiki/The_Golden_Crab
fn new_prinz() -> Crab {
    Crab::new(
        String::from(PRINZ),
        20,
        Color::new(255, 215, 0), // Golden!
        Diet::Shellfish,
    )
}

// Under the sea...
fn new_sebastian() -> Crab {
    Crab::new(
        String::from(SEBASTIAN),
        30, // faster than Prinz!
        Color::new_red(),
        Diet::Plants,
    )
}

fn new_crab(name: &str, speed: u32) -> Crab {
    Crab::new(
        String::from(name),
        speed,
        Color::new_blue(),
        Diet::Plants,
    )
}

/* Tests */

#[test]
fn part1_color_cross_no_panic() {
    let red = Color::new_red();
    let blue = Color::new_blue();
    let purple = Color::new(255, 0, 255);

    // When there is no overflow, it should not panic.
    let result = std::panic::catch_unwind(|| {
        assert_eq!(&Color::cross(&red, &blue), &purple);
    });
    assert!(result.is_ok());

    // When there would be overflow, it should not panic.
    let result = std::panic::catch_unwind(|| {
        assert_eq!(&Color::cross(&red, &red), &Color::new(254, 0, 0));
    });
    assert!(result.is_ok());
}

#[test]
fn part1_crab_new() {
    let c = new_prinz();

    assert_eq!(c.name(), PRINZ);
    assert_eq!(c.speed(), 20);
    assert_eq!(c.color(), &Color::new(255, 215, 0));
    assert_eq!(c.diet(), Diet::Shellfish);
}

#[test]
fn part1_beach_add_get_crab() {
    let mut beach = Beach::new();
    let c = new_prinz();

    beach.add_crab(c);
    let c = beach.get_crab(0);

    assert_eq!(c.name(), PRINZ);
    assert_eq!(c.speed(), 20);
    assert_eq!(c.color(), &Color::new(255, 215, 0));
    assert_eq!(c.diet(), Diet::Shellfish);
}

#[test]
fn part1_beach_iter_crabs() {
    let mut beach = Beach::new();
    for _i in 0..10 {
        beach.add_crab(new_prinz());
    }

    let mut num_crabs = 0;
    for c in beach.crabs() {
        assert_eq!(c.name(), PRINZ);
        num_crabs += 1;
    }
    assert_eq!(num_crabs, 10);
}

#[test]
fn part1_beach_size() {
    let mut beach = Beach::new();
    for i in 0..10 {
        assert_eq!(beach.size(), i);
        let prinz = new_prinz();
        beach.add_crab(prinz);
    }
}

#[test]
fn part1_beach_fastest_empty() {
    let beach = Beach::new();
    let fastest = beach.get_fastest_crab();
    assert!(fastest.is_none());
}

#[test]
fn part1_beach_fastest_fastest_first() {
    let mut beach = Beach::new();

    // Check empty beach.
    assert!(beach.get_fastest_crab().is_none());

    // Check when we add a fast crab...
    let c1 = new_sebastian();
    beach.add_crab(c1);

    match beach.get_fastest_crab() {
        Some(c) => assert_eq!(c.name(), SEBASTIAN),
        None => assert!(false),
    }

    // Now let's add a slower crab...
    let c2 = new_prinz();
    beach.add_crab(c2);

    match beach.get_fastest_crab() {
        Some(c) => assert_eq!(c.name(), SEBASTIAN),
        None => assert!(false),
    }
}

#[test]
fn part1_beach_fastest_fastest_second() {
    let mut beach = Beach::new();

    // Check empty beach.
    assert!(beach.get_fastest_crab().is_none());

    // Check when we add a slow crab...
    let c1 = new_prinz();
    beach.add_crab(c1);

    match beach.get_fastest_crab() {
        Some(c) => assert_eq!(c.name(), PRINZ),
        None => assert!(false),
    }

    // Now let's add a faster crab...
    let c2 = new_sebastian();
    beach.add_crab(c2);

    match beach.get_fastest_crab() {
        Some(c) => assert_eq!(c.name(), SEBASTIAN),
        None => assert!(false),
    }
}

#[test]
fn part1_crab_breeding() {
    let mut beach = Beach::new();

    beach.add_crab(new_prinz());
    beach.add_crab(new_sebastian());
    assert_eq!(beach.size(), 2);

    beach.breed_crabs(0, 1, String::from("Crabraham Lincoln"));
    assert_eq!(beach.size(), 3);
    if beach.size() < 3 {
        // Abort before running remaining tests
        return;
    }

    let parent1 = beach.get_crab(0);
    let parent2 = beach.get_crab(1);
    let child = beach.get_crab(2);

    assert_eq!(*child.name(), String::from("Crabraham Lincoln"));
    assert_eq!(child.speed(), 1);
    assert_eq!(*child.color(), Color::new(254, 215, 0));

    let parent1_diet = parent1.diet();
    let parent2_diet = parent2.diet();

    // If the implementation properly chooses a new diet, then the
    // new diet will sometimes be different from that of both parents.
    let mut found_different = false;
    for i in 0..1000 {
        // After their first inspired name choice... they gave up and started numbering their kids.
        beach.breed_crabs(0, 1, format!("Crab {}", i));
        let child = beach.get_crab(i + 3);
        if child.diet() != parent1_diet && child.diet() != parent2_diet {
            found_different = true;
            break;
        }
    }
    assert!(found_different);
}

#[test]
fn part1_crab_find_by_name() {
    let mut beach = Beach::new();

    beach.add_crab(new_sebastian());
    beach.add_crab(new_sebastian());

    let num_sebastians = beach.find_crabs_by_name(SEBASTIAN).len();
    assert_eq!(num_sebastians, 2);

    let num_prinz = beach.find_crabs_by_name(PRINZ).len();
    assert_eq!(num_prinz, 0);
}

#[test]
fn part2_crab_choose_recipe() {
    let cookbook = Cookbook::new();

    let prinz = new_prinz(); // eats shellfish
    let sebastian = new_sebastian(); // eats plants

    // Chowder is in the cookbook.
    let chowder = prinz.choose_recipe(&cookbook);
    assert!(chowder.is_some());
    assert_eq!(*chowder.unwrap().name(), String::from("chowder"));

    // But there are no vegetarian recipes in the cookbook.
    let none = sebastian.choose_recipe(&cookbook);
    assert!(none.is_none());
}

#[test]
fn part2_reef_add_take_prey() {
    let mut reef = Reef::new();
    let p = Clam::new();

    reef.add_prey(Box::new(p));
    assert_eq!(reef.population(), 1);

    let _ = reef.take_prey();
    assert_eq!(reef.population(), 0);
}

#[test]
fn part2_crab_discover_reefs() {
    let mut crab = new_prinz();
    let reef = Rc::new(RefCell::new(Reef::new()));

    // Only one reference exists: from this scope.
    assert_eq!(Rc::strong_count(&reef), 1);

    // Create a 2nd reference to the reef, and give it to `crab.discover_reef`.
    crab.discover_reef(Rc::clone(&reef));

    // Now there are two references to the reef.
    assert_eq!(Rc::strong_count(&reef), 2);
}

#[test]
fn part2_crab_hunt_empty_reef() {
    let mut crab = new_prinz();
    let reef = Rc::new(RefCell::new(Reef::new()));

    crab.discover_reef(reef);
    assert_eq!(crab.hunt(), false);
}

#[test]
fn part2_crab_hunt_success() {
    let mut crab = new_prinz();
    let reef = Rc::new(RefCell::new(Reef::new()));

    reef.borrow_mut().add_prey(Box::new(Clam::new()));
    assert_eq!(reef.borrow().population(), 1);

    // Prinz succeeded in hunting...
    crab.discover_reef(Rc::clone(&reef));
    assert_eq!(crab.hunt(), true);

    // ... and the reef should now be empty.
    assert_eq!(reef.borrow().population(), 0);
}

#[test]
fn part2_crab_hunt_incompatible_diet() {
    let mut crab = new_prinz();
    let reef = Rc::new(RefCell::new(Reef::new()));

    // Prinz eats shellfish, but not plants.
    reef.borrow_mut().add_prey(Box::new(Algae::new()));
    assert_eq!(reef.borrow().population(), 1);

    // Prinz failed in hunting...
    crab.discover_reef(Rc::clone(&reef));
    assert_eq!(crab.hunt(), false);

    // ... and the reef still contains the algae, which was released back to it.
    assert_eq!(reef.borrow().population(), 1);
}

#[test]
fn part2_crab_hunt_escaped_prey() {
    let mut crab = new_prinz();
    let reef = Rc::new(RefCell::new(Reef::new()));

    // Prinz eats shellfish, but this Shrimp will be able to escape (once).
    reef.borrow_mut().add_prey(Box::new(Shrimp::new(1)));
    assert_eq!(reef.borrow().population(), 1);

    crab.discover_reef(Rc::clone(&reef));

    // The first hunt fails. The Shrimp escapes.
    assert_eq!(crab.hunt(), false);
    assert_eq!(reef.borrow().population(), 1);

    // The second hunt succeeds. The Shrimp has no energy left.
    assert_eq!(crab.hunt(), true);
    assert_eq!(reef.borrow().population(), 0);
}

#[test]
fn part2_ocean_generate_algae_only() {
    for n_algae in 1..5 {
        // Create an ocean with no reefs...
        let mut ocean = Ocean::new();
        assert_eq!(ocean.reefs().len(), 0);

        // Generate a reef with that much algae only, in one reef.
        let reef = ocean.generate_reef(0, 0, 0, n_algae);
        assert_eq!(ocean.reefs().len(), 1);
        assert_eq!(reef.borrow().population(), n_algae as usize);

        // Prinz does not eat algae, so he should never succeed
        // hunting in a reef with only algae. Poor Prinz...
        let mut prinz = new_prinz();
        prinz.discover_reef(Rc::clone(&reef));
        assert_eq!(prinz.hunt(), false);

        // But Sebastian eats algae, so he should be fine.
        let mut sebastian = new_sebastian();
        sebastian.discover_reef(Rc::clone(&reef));
        assert_eq!(sebastian.hunt(), true);
    }
}

#[test]
fn part2_ocean_generate_algae_bountiful() {
    for n in 1..5 {
        // Create an ocean with no reefs...
        let mut ocean = Ocean::new();
        assert_eq!(ocean.reefs().len(), 0);

        // Generate a reef with that much algae only, in one reef.
        let reef = ocean.generate_reef(n, n, n, n);
        assert_eq!(ocean.reefs().len(), 1);
        assert_eq!(reef.borrow().population(), 4 * (n as usize));

        // Prinz eats clams and shrimp, so he should be fine.
        let mut prinz = new_prinz();
        prinz.discover_reef(Rc::clone(&reef));
        assert_eq!(prinz.hunt(), true);

        // Sebastian eats algae, so he should be fine.
        let mut sebastian = new_sebastian();
        sebastian.discover_reef(Rc::clone(&reef));
        assert_eq!(sebastian.hunt(), true);
    }
}

#[test]
fn part3_clans_get_clan_member_names_zero() {
    let beach = Beach::new();
    let clan_system = beach.get_clan_system();
    let names = clan_system.get_clan_member_names(CLAN_1);
    assert_eq!(names.len(), 0);
}

#[test]
fn part3_clans_get_clan_member_names() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 25);
    beach.add_crab(crab1);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    let clan_system = beach.get_clan_system();
    let names = clan_system.get_clan_member_names(CLAN_1);
    assert_eq!(names.len(), 1);
    assert_eq!(names[0], CRAB_1);
}

#[test]
fn part3_clans_get_clan_member_count_first() {
    let mut beach = Beach::new();
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_member_count(CLAN_1), 0);
    let crab1 = new_crab(CRAB_1, 25);
    beach.add_crab(crab1);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_member_count(CLAN_1), 1);
}

#[test]
fn part3_clans_get_clan_member_count_all() {
    let mut beach = Beach::new();
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_member_count(CLAN_1), 0);
    let crab1 = new_crab(CRAB_1, 25);
    let crab3 = new_crab(CRAB_3, 25);
    let crab4 = new_crab(CRAB_4, 25);
    beach.add_crab(crab1);
    beach.add_crab(crab3);
    beach.add_crab(crab4);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    beach.add_member_to_clan(CLAN_2, CRAB_4);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_member_count(CLAN_1), 1);
    assert_eq!(clan_system.get_clan_member_count(CLAN_2), 2);
}

#[test]
fn part3_clans_get_clan_count_zero() {
    let beach = Beach::new();
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_count(), 0);
}

#[test]
fn part3_clans_get_clan_count_all() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 25);
    let crab3 = new_crab(CRAB_3, 25);
    beach.add_crab(crab1);
    beach.add_crab(crab3);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_clan_count(), 2);
}

#[test]
fn part3_clans_get_largest_clan_id_empty_beach() {
    let beach = Beach::new();
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), None);
}

#[test]
fn part3_clans_get_largest_clan_id_first() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 25);
    let crab2 = new_crab(CRAB_2, 25);
    let crab3 = new_crab(CRAB_3, 25);
    beach.add_crab(crab1);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), Some(CLAN_1.to_string()));
    beach.add_crab(crab2);
    beach.add_member_to_clan(CLAN_1, CRAB_2);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), Some(CLAN_1.to_string()));
    beach.add_crab(crab3);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), Some(CLAN_1.to_string()));
}

#[test]
fn part3_clans_get_largest_clan_id_second() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 25);
    let crab3 = new_crab(CRAB_3, 25);
    let crab4 = new_crab(CRAB_4, 25);
    beach.add_crab(crab1);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), Some(CLAN_1.to_string()));
    beach.add_crab(crab3);
    beach.add_crab(crab4);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    beach.add_member_to_clan(CLAN_2, CRAB_4);
    let clan_system = beach.get_clan_system();
    assert_eq!(clan_system.get_largest_clan_id(), Some(CLAN_2.to_string()));
}

#[test]
fn part3_beach_get_winner_clan_empty_beach() {
    let mut beach = Beach::new();
    assert!(beach.get_winner_clan(CLAN_1, CLAN_2).is_err())
}

#[test]
fn part3_beach_get_winner_clan_tie() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 25);
    let crab2 = new_crab(CRAB_2, 25);
    let crab3 = new_crab(CRAB_3, 22);
    let crab4 = new_crab(CRAB_4, 28);
    beach.add_crab (crab1);
    beach.add_crab (crab2);
    beach.add_crab (crab3);
    beach.add_crab (crab4);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    beach.add_member_to_clan(CLAN_1, CRAB_2);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    beach.add_member_to_clan(CLAN_2, CRAB_4);
    assert_eq!(beach.get_winner_clan(CLAN_1, CLAN_2), Ok(None));
}


#[test]
fn part3_beach_get_winner_clan() {
    let mut beach = Beach::new();
    let crab1 = new_crab(CRAB_1, 24);
    let crab2 = new_crab(CRAB_2, 28);
    let crab3 = new_crab(CRAB_3, 30);
    let crab4 = new_crab(CRAB_4, 3);
    let crab5 = new_crab(CRAB_5, 30);
    beach.add_crab (crab1);
    beach.add_crab (crab2);
    beach.add_crab (crab3);
    beach.add_crab (crab4);
    beach.add_crab (crab5);
    beach.add_member_to_clan(CLAN_1, CRAB_1);
    beach.add_member_to_clan(CLAN_1, CRAB_2);
    beach.add_member_to_clan(CLAN_2, CRAB_3);
    beach.add_member_to_clan(CLAN_2, CRAB_4);
    beach.add_member_to_clan(CLAN_3, CRAB_5);
    assert_eq!(beach.get_winner_clan(CLAN_1, CLAN_2), Ok(Some(CLAN_1.to_string())));
    assert_eq!(beach.get_winner_clan(CLAN_2, CLAN_3), Ok(Some(CLAN_3.to_string())));
    assert_eq!(beach.get_winner_clan(CLAN_1, CLAN_3), Ok(Some(CLAN_3.to_string())));
}
