use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
println!("Esse programa escolheu um número aleatório, entre 1 a 100, e você tem que acertar qual ele escolheu.\n\
Vamos lá! Digite um número inteiro! \n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
	    let mut guess = String::new();

	    io::stdin()
    	        .read_line(&mut guess)
    	        .expect("A leitura da variável falhou, retorno Err");

	    println!(" ");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("INFORME UM NÚMERO!!!");
                    continue
                },
            };

	    match guess.cmp(&secret_number) {
        	        Ordering::Less => println!("Mais alto!"),
        	        Ordering::Greater => println!("Mais baixo!"),
        	        Ordering::Equal =>  {
                        println!("Você ganhou!");
                        break;
      }
    }
      println!(" ");
  }
}
