fn type_inference() {
    let x = 20;
    let y = 20;

    assert_eq!(x, y, "x and y must be equals");
}

fn my_str() {
    // &str: a borrowed, immutable view into text stored elsewhere.
    // String literals live in the program binary, so this costs nothing to create.
    let say_hello: &str = "Hello Matt";

    // String: an owned, growable text buffer allocated on the heap.
    // Use it when you need to modify the text (e.g. push_str) or own it.
    let mut say_bye: String = String::from("Good");

    say_bye.push_str(" bye");

    println!("{say_hello} / {say_bye} Matt")
}

pub fn data_tyes() {
    println!("Hello! I am data_tyes");
    type_inference();
    my_str();
}
