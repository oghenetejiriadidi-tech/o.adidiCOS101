use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Are you experienced? (yes/no):");
    io::stdin().read_line(&mut experience).unwrap();

    println!("Enter your age:");
    io::stdin().read_line(&mut age).unwrap();

    let experienced = experience.trim().eq_ignore_ascii_case("yes");
    let age: u32 = age.trim().parse().unwrap();

    let incentive = if experienced && age >= 40 {
        1_560_000
    } else if experienced && age >= 30 {
        1_480_000
    } else if experienced && age < 28 {
        1_300_000
    } else {
        100_000
    };

    println!("Annual incentive: ₦{}", incentive);
}
