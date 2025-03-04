fn call_me(num: u8) {
    for i in 0..num {
        match i.checked_mul(2) {
            Some(n) => println!("Ring! Call number: {} * 2 = {}", i, n),
            None => println!("Overflow detected for: {} * 2", i),
        };
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(130);
}
