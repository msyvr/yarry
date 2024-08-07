fn main() {
    // Pattern-match with `if-let` or `while`
    // Handy for when you want to do something only
    // for a particular variant.

    // Assume value actually comes from a map.
    let value = Some(42);

    {   
        // check if a variant satisfies the match    
        if let Some(inner) = value {
        println!("inner was {inner}");
        } else {
        println!("this is the failure case");
        }
    }

    // This is contrived; when this would be useful?
    // Maybe when Some(v) needs to happen in a particular order?
    // ... not sure about that, though.
    let values = vec![1,2,3,4,5];
    let mut iter = values.iter();

    while let Some(v) = iter.next() {
    println!("v = {v}");
    }
}
