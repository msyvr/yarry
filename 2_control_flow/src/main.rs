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
        // `msg` in this scope shadows the out-of-scope `msg` variable 
        let msg = "Ahoy, matey!";
        println!("{msg}");
    }
    println!("{msg}");

    let parrots = 5;
    let shipmates = 10;

    let legs_on_ship = {
        let parrot_legs = 2 * parrots;
        let human_legs = 2 * shipmates;
        println!("parrots: {parrots}, shipmates: {shipmates}");
        parrot_legs + human_legs
    };
    println!("parrots: {parrots}, shipmates: {shipmates} -> legs_on_ship: {legs_on_ship}");

    // Basic if-else
    let plunder = 10;
    if plunder > 5 {
        println!("A good haul");
    } else {
        println!("Just jetsam");
    }

    // Rust doesn't have the conditional/ternary operator,
    // so, to set a value conditionally:
    let is_crew_member = false;
    let greeting = if is_crew_member {
        "Ahoy!!! Welcome aboard!"
    } else {
        "Yarrrrr get off me ship"
    };
    println!("Crew member false! Greeting: {}", greeting);

    // Loops: loop-expressions; while-expressions; for-expressions

    // `loop` expressions: these are the basic infinite loops 
    // exit manually, using the `break` keyword
    let mut count = 0;
    loop {
        count += 1;
        println!("iteration {count}");
        if count >= 3 {
            break;
        }
    }

    // `while` expressions: have a halt condition
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("iteration {count}");
    }

    // for expressions: iterate over an iterable (typically, 
    // a collection of things, or a range)
    // range:
    for count in 1..=10 {
        println!("iteration {count}");
    }
    // collection:
    for prime in [2, 3, 5, 7, 11] {
        println!("{prime} is prime.");
    }

    // Loop values
    // Loop body must return (), so the last statement needs 
    // to end in a semicolon. To have the loop result in a 
    // value, emit the value using `break`.
    // In Rust, most expressions return values; if you try to
    // do that with a loop, you get a compiler error.
    // Also, while- and for-expressions may terminate prior to 
    // reaching a `break` statement. The compiler won't be
    // happy with the following:

    // let x = for count in 0..3 {
    // if count > 1 {
    //     break count * 2;
    // }
    // ^^^ produces a compiler error

    // ... however, the compiler is fine with emitting a 
    // value from an (infinite) `loop` expression:
    let mut count = 0;
    let x = loop {
        if count > 1 {
            break count * 2;
        }
        count += 1;
    };
    println!("The loop assigns {x} to var x.");

    fizz_buzz_while();
    fizz_buzz_for();
}

fn fizz_buzz_while() {
    let mut i = 1;
    while i <= 100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{i}");
        }
        i += 1;
    }
}

fn fizz_buzz_for() {
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{i}");
        }
    }
}
