// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.
//
fn my_function(word: Option<String>) {
    if let Some(content) = word {
        println!("The word is: {}", content);
    } else {
        println!("The optional word doesn't contain anything");
    }
}


fn main() {
    let optional_word = Some(String::from("rustlings"));
    let other_word = None;
    // TODO: Make this an if let statement whose value is "Some" type
    my_function(optional_word);
    my_function(other_word);

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers_vec.pop() {
        println!("current value: {:?}", integer.unwrap_or(0));
    }
}
