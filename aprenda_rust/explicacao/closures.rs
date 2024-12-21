
fn main() {
    let input = "56 65 58 48 59 56 87 23";

    let result: Vec<i32> = input
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .map(|n| n * 2)
        .collect();

    println!("{:?}", result);
}
