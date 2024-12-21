use std::io;

fn main() {
    println!("{:-^40}", "Calculadora");

    let mut s = String::new();
    let banner =
        "Digite uma sequencia de números\n\
        separados por vírgula\n\
        exemplo: 1,2,3,4,56";

    
    println!("{banner}");

    io::stdin()
        .read_line(&mut s) // retorna result
        .expect("Error reading console")

    // Criando uma função para utilizar no map
    fn clean(c: &str) -> &str {
        c.trim()
    }

    let nums: Vec<&str> = s.split(",")
        .map(clean)
        .collect();

    // Funções anonimas
    let nums: Vec<i32 = s
        .split(",")
        .map(|c| c.trim().parse().expect("Error"))
        .collect();

    // Somando os numeros
    let result: i32 = nums.iter().sum();

    println!("Você digitou {s}");

    // s.trim() remove os espaços em branco, s.len()verifica o tamanho
    println!("Quantidade de letras {}", s.trim().len());

    // Caso tenha emoji ou outros simbolos que contem mais caracteres
    println!("Quantidade de letras {}", s.trim().chars().count());


    println!("{}", "-".repeat(20));
}
