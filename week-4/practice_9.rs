use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter a:");
    io::stdin().read_line(&mut a).unwrap();
    println!("Enter b:");
    io::stdin().read_line(&mut b).unwrap();
    println!("Enter c:");
    io::stdin().read_line(&mut c).unwrap();

    let a: f64 = a.trim().parse().unwrap();
    let b: f64 = b.trim().parse().unwrap();
    let c: f64 = c.trim().parse().unwrap();

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots.");
    }
}
