// Regras de Borrowing
// 1. Podemos ter uma única referência caso ela seja mutável
// 2. Podemos ter várias quando são todas imutáveis

fn to_uppercase(text: &mut String) {
    *text = text.to_uppercase()
}

fn add_prefix(text: &mut String) {
    // *text = format!("FOO_{text}");
    text.push_str("_FOO");
}

fn main(){
    let mut name = "Bruno".to_string();
    to_uppercase(&mut name);
    add_prefix(&mut name);
}

// Regras de OWnership em Rust
// 1. Cada valor tem um dono (owner)
// 2. Só pode ter um único dono
// 3. Quando o dono sai de escopo o valor é limpo
// 4. A posso (ownership) pode ser movida a outro dono

fn say_hello(text: &String) {
    println!("HEllo, {text}");
}

fn say_goodbye(text: &String) {
    println!("Goodbye, {text}");
}

fn main() {

    // Usando Clone
    let name = "Ayslan".to_string(); // clone
    say_hello(name.clone());
    say_goodbye(name);

    //Melhor solução
    let name = "Ayslan".to_string(); // clone
    say_hello(&name);
    say_goodbye(&name);

    // --------VARIAVEIS OWNERSHIP ---------- // 
    // Utilizando a memoria heap
    // PENSE: a é dono da string 
    let a = String::from("Ayslan"); // No Copy

    // Movendo a propriedade, a passa ser invalidado e o b assume a propriedade
    let b = a;

    // Emprestando, a está emprestando o valor para o b
    let b = &a;

    // --------TIPOS DE COPY---------- //
    // a e b vão ter valores diferentes na memoria
    let a: i32 = 1; // Copy (i32, f54, bool, char)
    let b = a;

    // Fazendo uma referencia do b para o a
    let a: i32 = 1; 
    let b = &a;
    println!("O valor de A é {a}");
    println!("O valor de B é {b}");

    // println faz a referencia, porém se em algum momento utilizarmos uma função que não trata a referencia
    // devemos fazer da seguinte forma
    println!("O valor de A é {}", a);
    println!("O valor de B é {}", *b);
}
