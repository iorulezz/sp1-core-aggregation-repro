#![no_main]

sp1_zkvm::entrypoint!(main);

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;

            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }

            curr
        }
    }
}

fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let result = fibonacci(n);
    sp1_zkvm::io::commit(&result);
}
