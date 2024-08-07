fn main() {
    // Strust: named grouping of fields which can 
    // also have methods on it.

    struct PirateShip {
        name: String,
        captain: String,
        crew: Vec<String>,
        treasure: f64,
    }

    impl PirateShip {
        pub fn count_treasure(&self) -> f64 {
            // do some stuff to count it up
            self.treasure
        }

        pub fn mutiny(&mut self) {
            if self.crew.len() > 0 {
                self.captain = self.crew.pop().unwrap(); // .unwrap? String type!
                println!("Mutiny! {} is the new captain, ayyye.", self.captain)
            } else {
                println!("No crew, so no mutiny!");
            }
        }
    }

    let blackpearl = "Black Pearl".to_owned();
    let blackbeard = "Blackbeard".to_owned(); // to_owned vs clone because var must be String: see https://stackoverflow.com/questions/22264502/in-rust-what-is-the-difference-between-clone-and-to-owned
    let crew = vec!["Scurvy".to_owned(), "Rat".to_owned(), "Polly".to_owned()];
    let mut ship = PirateShip {
        name: blackpearl,
        captain: blackbeard,
        crew,
        treasure: 64.0,
    };
    println!("On Pirate Ship {} there are {} maties and {} treasure chests - ayyyyye!\nThe Captain is {} and the crew are {:?}.", ship.name, ship.crew.len(), ship.treasure, ship.captain, ship.crew);

    ship.mutiny()

}
