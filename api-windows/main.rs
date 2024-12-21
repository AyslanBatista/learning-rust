use std::ffi::CString; // necessário para converter as strings Rust (&str) em strings compatíveis com a API do Window
use winapi::um::winuser::{MessageBoxA, MB_OK}; // MessageBoxA: cria uma janela pop-up e MB_OK:constante para incluir um botão "OK"


fn main() {

    // Texto da mensagem
    let texto = CString::new("Esse é o meu texto").unwrap();

    // Titulo da caixa de mensagem
    let titulo = CString::new("Esse é meu titulo").unwrap();


    // Chama a função MessageBoxA, para exibir a caixa de texto
    // unsafe necessria porque a MessageBoxA é uma função do Windows API e não garante segurança
    unsafe {
        MessageBoxA(std::ptr::null_mut(), texto.as_ptr(), titulo.as_ptr(), MB_OK);
    }


}
