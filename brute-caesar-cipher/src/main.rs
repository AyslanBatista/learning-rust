/// Programa em Rust para quebrar a cifra de César por força bruta.

use std::io;

fn decrypt_caesar(texto_cifrado: &str, deslocamento: u8) -> String{
    //Converte todo texto para minúsculo
    let texto_minusculo = texto_cifrado.to_lowercase();
    let mut texto_descriptografado = String::new();

    for letra in texto_minusculo.chars(){
        if letra.is_ascii_alphabetic() {
            // Converte letra para número (a=0, b=1, etc)
            let numero_da_letra = letra as u8 - b'a';

            // Aplica o deslocamento para trás
            let posicao_anterior = numero_da_letra as i8 - deslocamento as i8;

            let mut posicao_corrigida = posicao_anterior;
            // Garante que ficamos no intervalo 0-25
            while posicao_corrigida < 0 {
                posicao_corrigida +=26;
            }

            // Converte número de volta para letra
            let letra_decriptada = (posicao_corrigida as u8 + b'a') as char;
            texto_descriptografado.push(letra_decriptada);
        } else {
            // Mantém outros caracteres que naõ são letras
            texto_descriptografado.push(letra);
        }
    }

    texto_descriptografado
}


fn main() {
    let mut texto_cifrado = String::new();

    println!("Digite o texto cifrado usando a cifra de César: ");

    // Armazenando o valor de entrada na variavel
    io::stdin().read_line(&mut texto_cifrado).expect("Erro ao ler a entrada");

    // Removendo espaços em braco
    let texto_cifrado = texto_cifrado.trim();

    println!("\n Tentando todas as chaves possíveis:\n");

    // Percorrendo todas as letras possíveis
    for deslocamento in 0..26 {
        let decrypted = decrypt_caesar(texto_cifrado, deslocamento);
        println!("Chave {}: {}", deslocamento, decrypted);
    }

}
