use std::time::Instant;

use pirates::{pirate_chores::PirateChore, PirateParty};

// Try to run this program and take a note of how long it takes to complete.
// Then, Instead of doing all the pirate work sequentially,
// divide it up to be run in parallell.
//
// Hints:
// - If you get a message from the compiler saying "borrowed value does not live long enough"
//   remember to try to use a thread::scope.
// - Another message you might get is 'closure may outlive the current function,'
//   This can indicate that you need to use a 'move' closure to capture variables by value
// - Divide a vector into chunks:
//   let work_chunks: Vec<&[PirateChore]> = work.chunks(chunk_size).collect();

fn main() {
    let work = PirateChore::new_random_roster();
    let before = Instant::now();

    let party = PirateParty::new(1);
    let pirate = party.get_pirate(0);
    for piece_of_work in work {
        pirate.do_chore(&piece_of_work);
    }
    party.println(&format!("Elapsed time: {:.2?}", before.elapsed()));
}

// 1. split the work into chunks
// 2. Create a thread scope
// 3. iterate over chunks
// 4. get pirate i from the pirate party
// 5. spawn a thread for this pirate
// 6. Within this thread, call do_chore on the pirate
