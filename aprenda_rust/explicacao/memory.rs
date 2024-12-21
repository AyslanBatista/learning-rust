// MEMORY STATIC - Alocar a variavel _A no espaço estatico de memoria do computador.
static _A: u32 = 37;

fn main() {

    // MEMORY STACK - Funciona como uma pilha, onde o programa vai poder remover e colocar elementos, tamanho fixo.
    let x = 5;
    let z = true;
    let numbers = [1, 2, 3];

    // MEMORY HEAP -  Usado para dados dinamicos, quando não sabemos a quantidade de elementos que vamos receber
    // (usuario digitando algo, valores recebido de um banco de dados, etc.).
    let users = get_users();

}
