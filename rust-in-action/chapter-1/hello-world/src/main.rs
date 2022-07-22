fn greet_world() {
    // The exclamation mark indicates a macro.
    // There is a lot of type detection going on under the hood so 
    // that arbitrary data types can be printed to the screen.
    println!("Hello, world!");

    // Variable binding in Rust uses the 'let' keyword.
    let southern_germany = "Grüß Gott!";

    // Unicode support is provided out of the box.
    let japan = "ハロー・ワールド";

    // Array literals use square brackets.
    let regions = [southern_germany, japan];

    // Many types have an .iter() method to return an iterator.
    for region in regions.iter() {
        // The ampersand indicates a reference.
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
