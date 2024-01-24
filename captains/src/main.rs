//Several captains order a pirate to do chores
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use pirates::pirate_chores::PirateChore;

static N_CAPTAINS: u32 = 4;

fn main() {
    let (tx, rx): (Sender<PirateChore>, Receiver<PirateChore>) = mpsc::channel();

    let mut captains = Vec::new();

    for _ in 0..N_CAPTAINS {
        let captain_tx = tx.clone();

        let captain = thread::spawn(move || {
            captain_tx.send(PirateChore::Scuttle).unwrap();
        });
        captains.push(captain);
    }
}
