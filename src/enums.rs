
pub enum Mensagem {
    Sair,
    Mover {x: i32, y: i32},
    Escrever(String),
}