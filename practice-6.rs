fn change_value(x: &mut i32) {
    *x = 100;
}

fn main() {
    let mut num = 20;
    change_value(&mut num);
    println!("New value: {}", num);
}
