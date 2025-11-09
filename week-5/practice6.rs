fn main() {
    let text1 = String::from("Semicolon");
    let text2 = String::from("Africa");
    let result = text1 + " " + &text2;

    println!("Result: {}", result);
}
