// Create 1 threads. In each thread print a random pirate name and its number
// The main thread should print "I am also a pirate"
use std::thread;

use pirates::Pirate;

// Create a thread:
// thread::spawn(||{})

//Join thread:
// thread_handle.join()

// Get a random pirate name:
// Pirate::get_random_name()

// How can we avoid the compiler error
// "closure may outlive the current function, but it borrows `i`,
// which is owned by the current function" ?
// Think about why the compiler protects us from this

fn main() {}
