use std::time::Instant;
mod pirate_chores;

use pirate_chores::PirateChore;

fn main() {
    let work = PirateChore::new_roster();
    let before = Instant::now();

    // Try to run this program and take a note of how long it takes to complete.
    // Then, Instead of doing all the pirate work sequentially, divide it between some of the cores on your computer.
    //
    // Hints:
    // - You can just specify the number of threads, or use some crate to find the number of cores on youir computer
    // - If you get a message from the compiler saying "borrowed value does not live long enough" try to use a thread::scope
    // - the 'chunks' method from std::slice is useful in dividing up the work
    for chore in work {
        chore.do_the_work(0);
    }

    //let threads = 8;
    //let work_chunks: Vec<&[PirateChore]> = work.chunks(work.len() / threads + 1).collect();
    // std::thread::scope(|scope| {
    //     for (i, chunk) in work_chunks.into_iter().enumerate() {
    //         scope.spawn(move || {
    //             for work in chunk {
    //                 work.do_the_work(i as u32);
    //             }
    //         });
    //     }
    // });
    println!("Elapsed time: {:.2?}", before.elapsed());
}
