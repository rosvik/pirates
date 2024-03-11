//Several captains order a pirate to do chores
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use pirates::pirate_chores::PirateChore;

static N_CAPTAINS: u32 = 4;
struct CaptainsOrder {
    captain_number: u8,
    chore: PirateChore,
}
// 1. create an mpsc channel
// 2. spawn a thread that receives orders from the channel
//    and just prints what it does and for which captain
// 3. Loop over the captains, (represented by just a number),
//    clone the channel for each captain and send an order to the channel
fn main() {}
