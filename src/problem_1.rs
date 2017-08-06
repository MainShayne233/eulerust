fn main() {
    let mut acc:   u32 = 0;
    let mut count: u32 = 1;
    while count < 1000 {
        if count % 3 == 0 || count % 5 == 0 {
            acc = acc + count;
        }
        count += 1;
    }
    println!("{}", acc);
}
