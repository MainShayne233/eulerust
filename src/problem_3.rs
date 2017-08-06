fn gcd(mut smaller_number: f64, mut greater_number: f64) -> f64 {
    if smaller_number > greater_number {
        return gcd(greater_number, smaller_number);
    }
    let mut remainder: f64 = greater_number % smaller_number;
    while remainder != 0.0 {
        greater_number = smaller_number;
        smaller_number = remainder;
        remainder      = greater_number % smaller_number;
    }
    return smaller_number;
}

fn is_prime(number: f64) -> bool {
    if number <= 3.0 {
        return number >= 2.0;
    } else if number == 5.0 {
        return true
    } else if gcd(30.0, number) != 1.0 {
        return false
    }
    let mut divisor: f64 = 7.0;
    while divisor <= number.sqrt() {
        if number % divisor          == 0.0 ||
           number % (divisor + 4.0)  == 0.0 ||
           number % (divisor + 6.0)  == 0.0 ||
           number % (divisor + 10.0) == 0.0 ||
           number % (divisor + 12.0) == 0.0 ||
           number % (divisor + 16.0) == 0.0 ||
           number % (divisor + 22.0) == 0.0 ||
           number % (divisor + 24.0) == 0.0 {
           return false;
        }
        divisor += 30.0;
    }
    return true;
}

fn next_prime(mut prime: f64) -> f64 {
    if prime % 2.0 == 0.0 {
        prime += 1.0;
    }
    loop {
        prime += 2.0;
        if is_prime(prime) {
            return prime;
        }
    }
}

fn main() {
    let mut factor: f64 = 2.0;
    let mut number: f64 = 600851475143.0;
    while number != 1.0 {
        if number % factor == 0.0 {
            number /= factor;
        } else {
            factor = next_prime(factor);
        }
    }
    println!("{}", factor);
}
