fn main() {
    // Ownership and the borrow checker are 
    // distinguishing features of the Rust language.

    // The compiler tracks when memory should be allocated 
    // and deallocated to ensure that *references remain 
    // valid*. It does so by tracking variables' lifetimes 
    // and ownership.

    // Example to demostrate the utility of lifetimes:
    // {
    //     let outer_ref: &u32; // <-- ref defined, not initialized
    //     {
    //         let inner_val: u32 = 10;
    //         outer_ref = &inner_val; // <-- fails to compile
    //     } // <-- inner_val goes out of scope here
    //     println!("outer_ref value: {}", outer_ref);
    // }

    // Another example:
    // fn naughty_function() -> &u32 {
    //     let x: u32 = 10; // local variable
    //     &x // ref to local var inside a function prevents compilation
    // }

    // Both examples above would compile in C... resulting in
    // use-after-free errors. Rust prevents these!

    // Lifetimes
    // - every reference is a "borrow"
    // - every borrow has a "lifetime"
    // - lifetime spans variable creation to destruction
    // The borrow checker ensures that reference lifetimes are
    // contained by the borrowed value's lifetime.
    // - lifetimes can be named; syntax: `'`prefix, eg `'a`

    let x: u32 = 10;
    let y = example(&x);
    println!("y from example({}): {}", &x, y);
    let z = example_idiomatic(&x);
    println!("z from example_idiomatic({}): {}", &x, z);

    // `static means for the duration of the program
    let msg: &'static str = "hello, world!";
    println!("{msg}");

    // Ownership
    // Can be changed: eg if variable moved out of scope, 
    // it gets a new owner.
    // An example often encountered by new Rusties:
    // let xs = vec![1,2,3]; // Vec<i32> doesn't implement the Copy trait
    // println!("defined xs as: {:?}", xs);
    // for x in xs { // ... thus, this implicit call to .into_iter() moves xs
    //     println!("looping through values: this one is {x}");
    // }
    // println!("Elements in xs: {}", xs.len()); // ... xs borrowed after move

    // Solution: use a reference to let the for loop borrow the Vec
    let xs = vec![1,2,3];
    println!("defined xs as: {:?}", xs);
    for x in &xs { // xs isn't moved when its reference is used
        println!("looping through values: this one is {x}");
    }
    println!("Elements in xs: {}", xs.len());

    // nb: some variables have the Copy trait ("are Copy") and
    // wouldn't require the reference implementation for the loop
    let x: u32 = 10;
    println!("x is {x}"); // x 'used' here, so its ownership moved?
    println!("x*2 is {}", x*2); // ... no, as x was copied
}

fn example<'a>(x: &'a u32) -> &u32 {
    let y: &'a u32 = &x;
    println!("y from example(x) is: {y}");
    y
}

fn example_idiomatic(x: &u32) -> &u32 {
    let y: &u32 = &x;
    println!("y from example_idiomatic(x) is: {y}");
    y
}

