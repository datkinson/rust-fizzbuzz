fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

fn main() {
    let default=10;
    if let Some(arg1) = std::env::args().nth(1) {
        if let Ok(x) = arg1.parse::<u32>() {
            fizzbuzz_to(x);
        } else {
            println!("Please supply an integer");
        }
    } else {
        println!("No range specified, defaulting to {}", default);
        fizzbuzz_to(default);
    }
}
