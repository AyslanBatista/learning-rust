fn main() {
    // Tupla, tamanho fixo de elementos e permite elementos de mais de um tipo
    let numbers = (1, 2, 3, 4.5);

    println!("{:?}", numbers);
    println!("{:?}", numbers.0); // numbers.0 Pegando o primeiro elemento da tupla

    let mut numbers_mutavel = (1, 2, 3);
    numbers_mutavel.0 = 50; // alterando o primeiro valor da tupla para 50
    println!("{:?}", numbers_mutavel);

    let (a, b, c, e) = numbers; // descompactando a tupla em outras variaveis
    println!("{}", a);

    numbers = (4, 5, 6, 8.6); // Para alterar a tupla toda, precisa ter o mesmo tamanho e os tipos correspondentes
    println!("{:?}", numbers)
}
