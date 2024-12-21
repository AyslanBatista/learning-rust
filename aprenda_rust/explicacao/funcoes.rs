#![allow(unused, dead_code)]

fn say_hello(name: &str, color: &str) -> &str {
    println!("Hello {name}, your color {color}");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    if x == 0 {
        return y;
    }

    x + y
}


fn main() {
    let result = say_hello("Ayslan", "red");
    say_hello("Ayslan", "blue");

    // let y tera o valor de retorno do bloco de codigo que é 99
    let y = {
        say_hello("Ayslan", "orange");
        let x = 5;
        99
    }

    println!("{:?}", y);

    let res: i32 = add_numbers(8, 9);
    println!("{res}");

}
