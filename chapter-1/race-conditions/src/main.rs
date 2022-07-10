// Brings multi-threading into local scope.
use std::thread;

fn main() {
    let mut data = 100;

    // Takes a closure as an argument.
    thread::spawn(|| {data = 500;});
    thread::spawn(|| {data = 1000;});

    println!("{}", data);

    // This throws many interesting errors.
}
