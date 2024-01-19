use std::{fmt::Display, thread, time::Duration};

use rand::Rng;

pub enum PirateChore {
    Keelhaul,
    Loot,
    Plunder,
    Pillage,
    Scuttle,
    WalkThePlank,
}

impl From<&PirateChore> for Duration {
    fn from(work: &PirateChore) -> Self {
        match work {
            PirateChore::Keelhaul => Duration::from_millis(1000),
            PirateChore::Loot => Duration::from_millis(3000),
            PirateChore::Plunder => Duration::from_millis(4000),
            PirateChore::Pillage => Duration::from_millis(6000),
            PirateChore::Scuttle => Duration::from_millis(30),
            PirateChore::WalkThePlank => Duration::from_millis(300),
        }
    }
}
impl Display for PirateChore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PirateChore::Keelhaul => write!(f, "Keelhauling"),
            PirateChore::Loot => write!(f, "Looting"),
            PirateChore::Plunder => write!(f, "Plundering"),
            PirateChore::Pillage => write!(f, "Pillaging"),
            PirateChore::Scuttle => write!(f, "Scuttling"),
            PirateChore::WalkThePlank => write!(f, "Walking the plank"),
        }
    }
}
impl PirateChore {
    fn new_from_number(number: u32) -> Self {
        match number {
            0 => PirateChore::Keelhaul,
            1 => PirateChore::Loot,
            2 => PirateChore::Plunder,
            3 => PirateChore::Pillage,
            4 => PirateChore::Scuttle,
            5 => PirateChore::WalkThePlank,
            _ => panic!("Unknown chore {}", number),
        }
    }

    pub fn new_random_roster() -> Vec<Self> {
        const NUMBER_OF_WORK_ITEMS: u32 = 32;
        let mut rng = rand::thread_rng();
        (0..NUMBER_OF_WORK_ITEMS)
            .collect::<std::vec::Vec<u32>>()
            .iter()
            .map(|_| PirateChore::new_from_number(rng.gen_range(0..6)))
            .collect()
    }
    pub fn new_roster() -> Vec<Self> {
        const NUMBER_OF_WORK_ITEMS: u32 = 32;

        (0..NUMBER_OF_WORK_ITEMS)
            .collect::<std::vec::Vec<u32>>()
            .iter()
            .map(|i| PirateChore::new_from_number(i % 6))
            .collect()
    }

    pub fn do_the_work(&self, i: u32) {
        println!("Thread {i}: {}", self);
        thread::sleep(self.into());
    }
}
