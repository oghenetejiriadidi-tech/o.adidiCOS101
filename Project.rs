use std::io;

fn area_trapezium(h: f32, b1: f32, b2: f32) -> f32 {
    (h / 2.0) * (b1 + b2)
}

fn area_rhombus(d1: f32, d2: f32) -> f32 {
    0.5 * d1 * d2
}

fn area_parallelogram(base: f32, altitude: f32) -> f32 {
    base * altitude
}

fn area_cube(side: f32) -> f32 {
    6.0 * side.powi(2)
}

fn volume_cylinder(radius: f32, height: f32) -> f32 {
    std::f32::consts::PI * radius.powi(2) * height
}

fn main() {
    println!("==== SHAPE CALCULATOR ====");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice: u32 = choice.trim().parse().unwrap();

    match choice {
        1 => {
            let (h, b1, b2) = get_three("height", "base1", "base2");
            println!("Area = {}", area_trapezium(h, b1, b2));
        }
        2 => {
            let (d1, d2) = get_two("diagonal1", "diagonal2");
            println!("Area = {}", area_rhombus(d1, d2));
        }
        3 => {
            let (base, altitude) = get_two("base", "altitude");
            println!("Area = {}", area_parallelogram(base, altitude));
        }
        4 => {
            let side = get_one("side");
            println!("Area = {}", area_cube(side));
        }
        5 => {
            let (r, h) = get_two("radius", "height");
            println!("Volume = {}", volume_cylinder(r, h));
        }
        _ => println!("Invalid choice!"),
    }
}

fn get_one(label: &str) -> f32 {
    println!("Enter {}:", label);
    let mut val = String::new();
    io::stdin().read_line(&mut val).unwrap();
    val.trim().parse().unwrap()
}

fn get_two(a: &str, b: &str) -> (f32, f32) {
    (get_one(a), get_one(b))
}

fn get_three(a: &str, b: &str, c: &str) -> (f32, f32, f32) {
    (get_one(a), get_one(b), get_one(c))
}
