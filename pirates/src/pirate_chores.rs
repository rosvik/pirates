use std::{fmt::Display, time::Duration};

use rand::Rng;

#[derive(PartialEq)]
pub enum PirateChore {
    Keelhaul,
    Loot,
    Plunder,
    Pillage,
    Scuttle,
    WalkThePlank,
    DividingTheBooty,
    SingAShanty,
    DrinkRum,
    Cleanup,
}

impl From<&PirateChore> for Duration {
    fn from(work: &PirateChore) -> Self {
        match work {
            PirateChore::Keelhaul => Duration::from_millis(2000),
            PirateChore::Loot => Duration::from_millis(2200),
            PirateChore::Plunder => Duration::from_millis(1500),
            PirateChore::Pillage => Duration::from_millis(3000),
            PirateChore::Scuttle => Duration::from_millis(300),
            PirateChore::WalkThePlank => Duration::from_millis(800),
            PirateChore::DividingTheBooty => Duration::from_millis(1400),
            PirateChore::SingAShanty => Duration::from_millis(600),
            PirateChore::DrinkRum => Duration::from_millis(900),
            PirateChore::Cleanup => Duration::from_millis(100),
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
            PirateChore::DividingTheBooty => write!(f, "Dividing the booty"),
            PirateChore::SingAShanty => write!(f, "Singing a shanty"),
            PirateChore::DrinkRum => write!(f, "Drinking Rum"),
            PirateChore::Cleanup => write!(f, "Cleaning up"),
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
            6 => PirateChore::DividingTheBooty,
            7 => PirateChore::SingAShanty,
            8 => PirateChore::DrinkRum,
            _ => panic!("Unknown chore {}", number),
        }
    }

    pub fn new_random_roster() -> Vec<Self> {
        const NUMBER_OF_WORK_ITEMS: u32 = 64;
        let mut rng = rand::thread_rng();
        (0..NUMBER_OF_WORK_ITEMS)
            .collect::<std::vec::Vec<u32>>()
            .iter()
            .map(|_| PirateChore::new_from_number(rng.gen_range(0..9)))
            .collect()
    }
    pub fn new_roster() -> Vec<Self> {
        const NUMBER_OF_WORK_ITEMS: u32 = 64;

        (0..NUMBER_OF_WORK_ITEMS)
            .collect::<std::vec::Vec<u32>>()
            .iter()
            .map(|i| PirateChore::new_from_number(i % 9))
            .collect()
    }
}
