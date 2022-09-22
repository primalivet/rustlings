// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    // string2 moved here as it's passed by reference to result so when it lives inside the block
    // it dies when the block end but as it's passed by reference result depends on it. and result
    // is used in println so we could have moved println into the block, thats another solution
    let string2 = String::from("xyz"); 
    {
        // string2 originally lived here.
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
