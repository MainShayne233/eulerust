fn reverse(string: String) -> String {
    let mut reversed_string: String = String::from("");
    for i in (0..string.len()).rev() {
        reversed_string.push(string.chars().nth(i).unwrap());
    }
    return reversed_string;
}

fn is_palindrome(number: u32) -> bool {
    let mut string: String = number.to_string();
    string = reverse(string);
    let reversed = string.parse().unwrap();
    return number == reversed;
}

fn main() {
    let mut largest_palindrome: u32 = 0;
    let mut product: u32;
    for x in 100..999 {
        for y in x..999 {
            product = x * y;
            if product > largest_palindrome && is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }
    println!("{}", largest_palindrome);
}
