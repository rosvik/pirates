use pirate_party::pirate::PirateParty;

// Implement a pirate party where 8 pirates talk over each other
// To avoid the implicit locking of stdout that println! does, use writeln! instead
// Each pirate should utter 5 pirate terms

// Create a new Party like so:
// let party = PirateParty:new(number_of_pirates);

// Get a pirate:
// party.get_pirate(pirate_number);

// make a pirate say something:
// pirate.utter_random_phrase();

fn main() {
    let party = PirateParty::new(8);

    std::thread::scope(|scope| {
        for t in 0..8 {
            let pirate = party.get_pirate(t);
            scope.spawn(move || {
                for _ in 0..5 {
                    pirate.utter_random_phrase();
                }
                //pb.finish_with_message("Done");
            });
        }
    });
}
