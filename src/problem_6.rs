fn main() {
    let mut sum_of_squares: i32 = 0;
    let mut square_of_sum: i32 = 0;
    for number in 1..101 {
        sum_of_squares += i32::pow(number, 2);
        square_of_sum += number;
    }
    square_of_sum = i32::pow(square_of_sum, 2);
    println!("{}", (sum_of_squares - square_of_sum).abs());
}
