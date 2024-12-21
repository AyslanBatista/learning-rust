fn main() {

    // string slice, string reference
    // A variavel nome vai conter uma "referencia" de onde a string começa [inicio, len]
    let nome: &str = "Ayslan";

    // HEAP
    // Heap String
    // String Dinâmica
    let mut s = String::new();
    s.push('a');
    s.push('y');
    s.push('s');
    s.push('l');
    s.push('a');
    s.push('n');
    println!("{s}");

    //Pegando uma string slice, e jogando para dentro de uma String
    s.push_str("Ayslan")
    s.push_str(" ")
    s.push_str("Batista")
    println!("{s}");

    // Outra forma de criar uma String (mais comum)
    let s = "Ayslan".to_string();

    //Outra forma
    let s: String = String::from("Ayslan")

    //Outra forma - into() precisa ter a anotação de tipo, pois será convertido
    let s: String = "Ayslan".into();

    //Criando uma string apartir de um objeto interavel
    let nome = ['a', 'y', 's', 'l', 'a', 'n'];
    let s = String::from_iter(nome);

    
}
