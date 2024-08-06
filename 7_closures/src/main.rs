fn main() {
    // Rust closures enable anonymous functions.
    let y: u32 = 10;
    let annotated = |x: u32| -> u32 { x + y };
    let inferred = |x| x + y;

    println!("annotated(32): {}", annotated(32));
    println!("inferred(32): {}", inferred(32));
    
    // Basic syntax:
    // pipes around the parameter list followed by
    // the expression for the desired return value
    // - a no-arg closure (||) is just an empty param list

    // Closures can reference values from their outer scope.
    // They can also capture the outer values and use them;
    // eg, for counters - and! the captured var remains valid
    // in its original scope, the outer scope of the closure:
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };

    println!("count is {}", increment());
    println!("count is {}", increment());
    println!("count is {}", increment());
    println!("count after calling increment 3x is {}", count); // still valid!

    // Can return closures from functions. If any outer scope
    // variables are captured by such a closure, they'll need
    // to be moved into the closure. 
    // Both the print_msg and make_counter functions return an
    // `impl` of a trait. (Will see more about traits later.)
    // For now, think of a trait as an interface: defines what
    // can be done, but doesn't specify the type to which it applies.
    // Functions can have three traits: Fn, FnMut, FnOnce.
    // Since each closure has its own type, closures generally return:
    // `impl (Fn || FnMut || FnOnce)`
    // `Fn` can be used as `FnMut` or `FnOnce`; likewise, `FnMut`
    // can be used as `FnOnce`. The inverse is not true.

    let f = print_msg("msg: hello, world"); // nothing printed yet
    f(); // invoke the function, ie the closure returned by print_msg

    // Another example: a function that makes a counter.
    let mut counter = make_counter();

    println!("count is {}", counter());
    println!("count is {}", counter());
    println!("count is {}", counter());

    // Independent exercises
    // Add two numbers.
    let x: f32 = 5.3;
    let y: f32 = 13.5;
    let add = add_two(x, y);
    println!("add_two for {} + {} = {}", x, y, add());
    // Increment an initial value by the stated increment.
    let incr: f32 = 10.0;
    let mut start: f32 = 100.0;
    for i in 0..=5 {
        let bumped = add_incr(start, incr);
        println!("add_incr for incr = {}, started at {}, on bump {:?} = {}", incr, start, i, bumped());
        start = bumped();
    };
}

// Why does this function require lifetime 'a? 
// Does + 'a signify that the closure func and lifetime are both returned?
fn print_msg<'a>(msg:&'a str) -> impl Fn() + 'a {
    let printer = move || { // move ownership of msg to printer closure
        println!("{msg}");
    };
    printer
}

fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    let increment = move || {
        count += 1;
        count
    };
    increment
}

fn add_two(x: f32, y: f32) -> impl FnOnce() -> f32 {
    let adder = move || {
        x + y
    };
    adder
}

fn add_incr(start: f32, incr: f32) -> impl Fn() -> f32 {
    let bumper = move || {
        start + incr
    };
    bumper
}