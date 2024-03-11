pub mod pirate_chores;

use std::{thread, time::Duration};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use pirate_chores::PirateChore;
use rand::Rng;

pub const PIRATE_TERMS: [&str; 15] = [
    "Avast Ye",
    "Yarr",
    "I am a free prince",
    "Damnation seize my soul if I give you quarters",
    "Grog!",
    "Savvy?",
    "one bottle o'rum for me",
    "As to hanging, it is no great hardship",
    "damn them for a pack of crafty rascals",
    "Sink me",
    "yo ho ho",
    "blow the man down",
    "Damn my blood",
    "Let's jump on board, and cut them to pieces",
    "I'm Guybrush Threepwood, and I'm a mighty pirate.",
];

pub const PIRATE_NAMES: [&str; 8] = [
    "Long John Silver",
    "Captain Hook",
    "Guybrush Threepwood",
    "Blackbeard",
    "Maximus Darkskull",
    "Oscar Foul",
    "Francis Drake",
    "Mary Read",
];

#[derive(Clone)]
pub struct Pirate {
    progress: ProgressBar,
}
impl Drop for Pirate {
    fn drop(&mut self) {
        self.finish();
    }
}
impl Pirate {
    pub fn new(progress: ProgressBar) -> Self {
        Self { progress }
    }

    pub fn utter_random_phrase(&self) {
        let msg = String::from(PIRATE_TERMS[rand::thread_rng().gen_range(0..PIRATE_TERMS.len())]);
        self.progress.set_message(msg);
        self.progress.inc(1);
        thread::sleep(rand::thread_rng().gen_range(Duration::from_secs(1)..Duration::from_secs(3)));
    }

    pub fn do_chore(&self, chore: &PirateChore) {
        let chore_description = format!("{}", chore);
        self.progress.set_message(chore_description);
        self.progress.inc(1);
        thread::sleep(chore.into());
    }
    pub fn finish(&self) {
        self.progress.finish();
    }
    pub fn get_random_name() -> &'static str {
        PIRATE_NAMES[rand::thread_rng().gen_range(0..8)]
    }
}
pub struct PirateParty {
    pirates: Vec<Pirate>,
    mp: MultiProgress,
}

impl PirateParty {
    pub fn new(participants: usize) -> Self {
        let mp = MultiProgress::new();

        let style = ProgressStyle::with_template("{prefix:.bold.dim} {msg}").unwrap();
        let pirates: Vec<Pirate> = (0..participants)
            .collect::<std::vec::Vec<usize>>()
            .iter()
            .map(|_| Self::create_pirate(&mp, &style))
            .collect();
        Self { pirates, mp }
    }
    pub fn get_pirate(&self, i: usize) -> &Pirate {
        &self.pirates[i]
    }
    pub fn create_pirate(mp: &MultiProgress, style: &ProgressStyle) -> Pirate {
        let pb = mp.add(ProgressBar::new(5));
        pb.set_style(style.clone());
        pb.set_prefix("\u{1F480}");
        Pirate::new(pb)
    }

    pub fn println(&self, msg: &str) {
        self.mp.println(msg).unwrap();
    }
}
