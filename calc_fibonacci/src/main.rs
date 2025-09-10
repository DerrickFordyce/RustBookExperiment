// quick and dirty

fn main() {
    for num in 0..21 {
        println!("Fibonacci number at sequence {num} is: {}", calc_fib(num));
    }
}

fn calc_fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;

    if n == 0 { return 0 };

    for _num in 1..n {
        c = a + b;
        a = b;
        b = c;
    }

    b
}
