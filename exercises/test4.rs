// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!
macro_rules! my_macro {
    ($s:expr) => {{ 
        let mut v = String::from("Hello ");
        v.push_str($s);
        v
    }}
}
fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
