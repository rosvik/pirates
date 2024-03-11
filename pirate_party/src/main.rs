// Implement a pirate party where 8 pirates talks simultaneously
// Each pirate should utter about 5 pirate terms

// A PirateParty is created with PirateParty::new(n_partygoers)

// Get a pirate from a party with party.get_pirate(pirate_number)

// Then create a new thread for each Pirate and make the pirate utter random phrases there
// The Pirate should not be cloned or copied

// A thread scope is created like this:
// thread::scope(|my_scope|{})

// A thread is created on a scope like this:
// scope.spawn(|| {})

use pirates::PirateParty;

fn main() {
    // 1. Create a new PirateParty with 8 members
    // 2. Create a thread scope
    // 3. Inside this scope create 8 threads with their own pirate
    // 4. use a for loop within these threads to make the associated pirate utter about 5 random phrases
}
