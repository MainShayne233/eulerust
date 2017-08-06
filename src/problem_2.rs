fn main() {
    let mut first_number:  u32 = 1;
    let mut second_number: u32 = 2;
    let mut acc:           u32 = 0;
    let mut temp:          u32;
    while second_number <= 4000000 {
        if second_number % 2 == 0 {
            acc += second_number;
        }
        temp = first_number;
        first_number = second_number;
        second_number += temp;
    }
    println!("{}", acc);
}
