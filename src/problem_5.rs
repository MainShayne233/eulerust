mod math;

fn main() {
    let mut arr: [[f64; 2]; 8] = [[0.0; 2]; 8];
    let mut prime: f64 = 2.0;
    let mut factor: f64;
    let mut product: f64 = 1.0;
    for i in 0..8 {
        arr[i][0] = prime;
        prime = math::next_prime(prime);
    }
    for f in 2..21 {
      factor = f64::from(f);
      for i in 0..8 {
          let mut count: f64 = 0.0;
          prime = arr[i][0];
          while factor % prime == 0.0 && factor > 1.0 {
              factor /= prime;
              count += 1.0;
          }
          if count > arr[i][1] {
              arr[i][1] = count;
          }
      }
    }
    for i in 0..8 {
        product *= arr[i][0].powf(arr[i][1]);
    }
    println!("{}", product);
}
