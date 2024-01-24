use std::time::Instant;

use pirates::{pirate_chores::PirateChore, PirateParty};

fn main() {
    let work = PirateChore::new_random_roster();
    let before = Instant::now();

    // Try to run this program and take a note of how long it takes to complete.
    // Then, Instead of doing all the pirate work sequentially,
    // divide it up to be run in parallell.
    //
    // Hints:
    // - the 'chunks' method from std::slice is useful in dividing up the work
    // - If you get a message from the compiler saying "borrowed value does not live long enough"
    // remember to try to use a thread::scope. Another issue
    // - Another message you might get is 'closure may outlive the current function,'
    //   This can indicate that you need to use 'move'-semantics in the thread closure

    let threads = 8;
    let chunk_size = work.len() / threads;
    let work_chunks: Vec<&[PirateChore]> = work.chunks(chunk_size).collect();
    let pirate_party = PirateParty::new(threads);
    std::thread::scope(|scope| {
        for (i, chunk) in work_chunks.into_iter().enumerate() {
            let pirate = pirate_party.get_pirate(i);
            scope.spawn(move || {
                for work in chunk {
                    pirate.do_chore(work)
                }
            });
            pirate.finish();
        }
    });
    println!("Elapsed time: {:.2?}", before.elapsed());
}
