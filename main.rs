use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*; // Colored è una dependecy di cargo. Ovvero la libreria

fn main()
{
    println!("Indovina il numero!");
    let mut rng = rand::thread_rng();
    let n_segreto = rng.gen_range(0..101);

    
    
    let mut messaggio: String = n_segreto.to_string(); // Questo è perchè colored funziona solo con tipi STRINGA
    
    // Per convertire in stringa, possiamo usare tranquillamente il .to_string().
    
    println!("il numero è {}", messaggio.yellow());

    loop //normale guessing game 
    {
        println!("inserisci un numero!");
        let mut numero: String = String::new();

        io::stdin().read_line(&mut numero).expect("Fallito la lettura del numero!");

        let numero: u32 = match numero.trim().parse()
        {
            Ok(n) => n,
            Err(_) => {
                println!("inserisci un {}", "Numero".red()); // per inserire i colori basta usare un metodo affianco ad un tipo stringa, infatti .red() ecc.
                continue;
            },
        };

        println!("Hai indovinato: {}", numero);

        match numero.cmp(&n_segreto){
            Ordering::Equal => {
                println!("{}", "Hai Indovinato!".green());
                break;
            },
            Ordering::Greater => println!("{}", "Troppo Grande!".red()),
            Ordering::Less => println!("{}", "Troppo Piccolo!".red()),
        }    
    }

}
