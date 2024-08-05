use std::collections::HashMap;
fn main() {

    // Heap allocation useful for:
    // - size unknown at compile time
    // - define independently of scope

    // Allocation methods: boxed values, vecs, other collection types

    // Boxed values
    let x: Box<u32> = Box::new(42);
    let y = Box::<f64>::new(4.2);
    println!("construct x and y on the heap: {x}, {y}");

    let x: Box<u32> = Box::new(42);
    let y = *x; // y is now a u32 on the stack; x still on the heap
    println!("construct x on the heap: {}\n... and y (from x) on the stack: {}", *x, y);

    // Vecs: basically, lists

    // The long way...
    let mut parrots: Vec<&str> = Vec::new();
    parrots.push("Shivers");
    parrots.push("Tweety");
    parrots.push("Dinner");
    println!("parrots list: {:?}", parrots);
    // ... and inline
    let parrots = vec!["Shivers", "Tweety", "Dinner"];
    println!("parrots list (inline construction): {:?}", parrots);

    // Lists are iterables:
    for parrot in parrots.iter() {
        println!("{} says hi.", parrot);
    }

    // Other collection types
    // see std::collections for all available types and when to use each
    // sample: Vec, HashMap, BTreeMap, HashSet, BTreeSet

    // Construct HashMap with pirate ship names as keys
    // and their crew sizes as values. Iterate over the hashmap
    // and print each ship name and its crew size.

    let mut fleet: HashMap<&str, u32> = HashMap::new();
    fleet.insert("Black Pearl", 13);
    fleet.insert("White Pearl", 3);
    fleet.insert("Grey Pearl", 8);
    for ship in fleet.iter() {
        println!("Ship {} has a crew of {}", ship.0, ship.1);
    }
}
