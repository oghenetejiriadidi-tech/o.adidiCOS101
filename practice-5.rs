fn change_value(mut x: i32) {
    x = 100;
    println!("Inside function: x = {}", x);
}

fn main() {
    let x = 20;
    change_value(x);
    println!("Outside function: x = {}", x);
}
