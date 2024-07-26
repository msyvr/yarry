fn main() {

    // Every control flow mechanism takes a block.
    // Blocks denoted by {...}. Notably, blocks:
    // 1. delimit a scope
    // 2. return the value of the last expression in the block
    // note: beyond the scope of this chapter but a common use
    // of blocks is to release resources: lock a mutex at the
    // beginning of a block and, when the block ends, it will 
    // be released (like `with lock:` in Python)

    let msg = "Hello, world";
    {
        // pirate block
        // `msg` in this scope shadows the out-of-scope `msg` variable 
        let msg = "Ahoy, matey!";
        println!("{msg}");
    }

    // Next, the utility of shadowing out-of-scope vars to 
    // manipulate and use them, but retain the original values.

    let parrots = 5;
    let shipmates = 10;

    let legs_on_ship = {
        let parrot_legs = 2 * parrots;
        let human_legs = 2 * shipmates;
        parrot_legs + human_legs
    };

    // If-else
    let plunder = 10;
    if plunder > 5 {
        println!("A good haul");
    } else {
        println!("Just jetsam");
    }

    // Rust doesn't have the conditional/ternary operator,
    // this is the way to set a value conditionally:

    let is_crew_member = true;
    let greeting = if is_crew_member {
        "Ahoy!!! Welcome aboard!"
    } else {
        "Yarrrrr get off me ship"
    };
    println!("{}", greeting);

    // Loops: these are the basic infinite loops; exit using 
    // the `break` keyword.
    let mut count = 0;

    loop {
        count += 1;

        println!("iteration {count}");

        if count >= 10 {
            break;
        }
    }

    // while expressions: like loop but with a halt condition
    let mut count = 0;

    while count < 10 {
        count += 1;
        println!("iteration {count}");
    }

    // for expressions: iterate over an iterable (typically, a collection
    // of things or a range)

    for count in 1..=10 {
        println!("iteration {count}");
    }

    for prime in [2, 3, 5, 7, 11] {
        println!("{prime} is prime.");
    }

    // Loop values: in Rust, most expressions return values but
    // if you try to do that with a loop, you get a compiler error
    // - the loop body must return a () so the last statement needs
    // to end in a semicolon
    // - to have the loop result in a value: emit the value using `break`

    // let x = for count in 0..3 {
    // if count > 1 {
    //     break count * 2;
    // }
    // ^^^ get compiler error

    let mut count = 0;

    let x = loop {
        if count > 1 {
            break count * 2;
        }

        count += 1;
    };
}

