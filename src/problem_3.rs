mod math;

fn main() {
    let mut factor: f64 = 2.0;
    let mut number: f64 = 600851475143.0;
    while number != 1.0 {
        if number % factor == 0.0 {
            number /= factor;
        } else {
            factor = math::next_prime(factor);
        }
    }
    println!("{}", factor);
}
