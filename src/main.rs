use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut life = 5;
    let random_secret_number = rand::thread_rng().gen_range(1..=10);
    println!("[$] Bem-vindo ao jogo de adivinhar o número secreto gerado!");
    println!("[$] Insira o número que deseja chutar: ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("[!] Houve um erro em ler a linha!");

        let guess: u32 = match guess
            .trim()
            .parse(){
            Ok(num)=>num,
            Err(_)=> continue,
        };
        match guess.cmp(&random_secret_number) {
            Ordering::Less => println!("[!] Menor do que o valor secreto gerado!"),
            Ordering::Equal => {
                println!("[#] Igual ao valor secreto gerado! Parabéns, você venceu o jogo!");
                break;
            },
            Ordering::Greater => println!("[!] Maior que o valor secreto gerado!"),
        }
        life -= 1;
        match 0.cmp(&life) {
            Ordering::Equal => {
                println!("[!] Você zerou suas vidas. O programa será encerrado, boa sorte na próxima!");
                break;
            },
            _ => {
                println!("[!] Você errou, vidas restantes: {life}! Tome cuidado, tente outro número: ");
            }
        }
    }

}
