fn main() {
    let mut count = 0;

    loop {
        count += 1;

        if count == 3 {
            println!("Skipping number 3");
            continue;
        }

        println!("Count: {}", count);

        if count == 5 {
            println!("Breaking loop");
            break;
        }
    }
}
