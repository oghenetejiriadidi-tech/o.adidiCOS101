
    use std::io;

fn main() {
    println!("Welcome to Semicolon Canteen!");
    println!("=====================================");
    println!("Menu:");
    println!("P = Poundo Yam / Edinkaiko Soup      - ₦3,200");
    println!("F = Fried Rice & Chicken             - ₦3,000");
    println!("A = Amala & Ewedu Soup               - ₦2,500");
    println!("E = Eba & Egusi Soup                 - ₦2,000");
    println!("W = White Rice & Stew                - ₦2,500");
    println!("=====================================");

    let mut total_bill = 0;
    let mut orders = Vec::new();

    loop {
        println!("\nEnter the code of your choice (P, F, A, E, W) or Q to quit:");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice = choice.trim().to_uppercase();

        if choice == "Q" {
            break;
        }

        let (meal_name, price) = match choice.as_str() {
            "P" => ("Poundo Yam / Edinkaiko Soup", 3200),
            "F" => ("Fried Rice & Chicken", 3000),
            "A" => ("Amala & Ewedu Soup", 2500),
            "E" => ("Eba & Egusi Soup", 2000),
            "W" => ("White Rice & Stew", 2500),
            _ => {
                println!("Invalid choice. Try again.");
                continue;
            }
        };

        println!("How many portions of {} would you like?", meal_name);

        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read quantity");

        let quantity: i32 = match quantity.trim().parse() {
            Ok(num) if num > 0 => num,
            _ => {
                println!("Invalid quantity. Please enter a positive number.");
                continue;
            }
        };

        let subtotal = price * quantity;
        total_bill += subtotal;

        orders.push((meal_name.to_string(), quantity, subtotal));

        println!(
            "{} x {} = ₦{} added to your order.",
            meal_name, quantity, subtotal
        );
    }

    if orders.is_empty() {
        println!("No items ordered. Goodbye!");
        return;
    }

    // Apply discount for big spenders
    let discount = if total_bill >= 10000 {
        total_bill as f32 * 0.1 // 10% discount
    } else {
        0.0
    };

    let final_total = total_bill as f32 - discount;

    println!("\n=========== ORDER SUMMARY ===========");
    for (meal, qty, subtotal) in &orders {
        println!("{} x {} = ₦{}", meal, qty, subtotal);
    }

    println!("-------------------------------------");
    println!("Subtotal: ₦{}", total_bill);
    println!("Discount: ₦{:.0}", discount);
    println!("Total: ₦{:.0}", final_total);
    println!("=====================================");
    println!("Thank you for dining with Semicolon Canteen!");
}

