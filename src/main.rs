use crate::enums::Mensagem;

mod subtracao;
mod multiplicacao;
mod divicao;
mod adicao;

mod enums;

struct Retangulo{
    largura: u32,
    altura: u32,
}

impl Retangulo{
    fn area(&self) -> u32{
        self.largura * self.altura
    }
}

struct Ola{

}

impl Ola{
    fn Hello() -> String {
        return "Olá".parse().unwrap();
    }
}

fn main() {

    let a= adicao::chamar_somar(200,200);
    let b= subtracao::subtracao(10, 10);
    let c= multiplicacao::multiplicacao(30, 10);
    let d= divicao::divicao(10,5);

    println!("Valores: {}, {}, {}, {}", a,b,c,d);

    // Pattern matching
    let mensagem = Mensagem::Escrever(String::from("Olá, Rust!"));

    match mensagem {
        Mensagem::Sair => {
            println!("Encerrando o programa...");
        }
        Mensagem::Mover { x, y } => {
            println!("Movendo para x: {}, y: {}", x, y);
        }
        Mensagem::Escrever(texto) => {
            println!("Texto recebido: {}", texto);
        }
    }

    // Exemplo de destructuring de uma tupla
    let tupla = (10, 20, 30);

    // Desestruturando a tupla para extrair seus elementos individuais
    let (a, b, c) = tupla;

    println!("Elementos da tupla: a = {}, b = {}, c = {}", a, b, c);

    let array : [i32; 2] = [10, 10];

    let [h, g] = array;

    println!("Valores h = {}, g = {}", g, h);

    let retangulo = Retangulo { largura: 10, altura: 20 };
    println!("A área do retângulo é: {}", retangulo.area());

    let hello = Ola::Hello();

    println!("{:?}", hello);

}

