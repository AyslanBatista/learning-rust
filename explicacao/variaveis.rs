// Definindo uma constante fora do escopo
const SECONDS_IN_MINUTE: u32 = 60;

fn main() {
    // escopo

    // constante dentro do escopo
    const MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * SECONDS_IN_HOUR;

    let total = 30; // variavel imutavel
    println!("Trabalhou {} horas", total);

    {
        // escopo interno
        let total = 25;
        println!("Trabalhou {} horas", total);
    } // fim

    let mut valor = 40; // variavel mutavel
    println!("Trabalhou {} horas", valor);
    valor = 50;
    println!("Trabalhou {} horas", valor);
} // fim
