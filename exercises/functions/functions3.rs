// functions3.rs
// Make me compile! Scroll down for hints :)

fn main() {
    let x: i32 = 20;
    call_me(x);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}




























// This time, the function *declaration* is okay, but there's something wrong
// with the place where we're calling the function.
