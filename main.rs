use std::io;
use std::io::Write;
// use rand::Rng; qui non serve

// "use" ci permette di avvisare cosa stiamo importando e di che libreria
// std::io, stiamo importando la libreria di input/output
// rand::Rng, stiamo importando la libreria di random

// ATTENZIONE! LA LIBRERIA rand::Rng è una libreria CHE METTIAMO TRAMITE IL CARGO.TOML!!!

fn main() // main è il nome del programma ed fn è "funzione"
{ //gli argomenti non sono passati. se voglio usarli, uso: std::env::args()

    //print!("Inserisci un numero: "); 
    //sarebbe più corretto usare print, ma ciò provoca l'attesa di un Input, prima di fare il print.
    // per fare un normale print dobbiamo specificare un pacchetto di std::io... std::io::Write
    // di conseguenza il codice basilare corretto è:
    print!("Inserisci un numero: ");
    io::stdout().flush().unwrap(); //unwrap è quasi un equivalente di .expect(), si può usare infatti al suo posto. flush invece è come readline ecc.

    let mut guess = String::new(); 
    // let è la dichiarazione di una variabile che non può essere modificata. DEFINISCE UNA VARIABILE
    // mut rende la variabile modificabile
    // String è la dichiarazione
    // new è una funzione che crea una nuova stringa vuota. SOLO STRINGA
    // La combo String::new(), è una stringa a capienza variabile.
    // inoltre :: indica che il new è associata al tipo String
    // altrimenti la variabile avrà il tipo assegnato dal compilatore in automatico.
    // altrimenti usa: "let mut guess: String = String::new();"

    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // read_line è una funzione che legge una riga di input da tastiera

    // si può scrivere anche:
    /*
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");    
    */

    // io.stdin() ritorna un istanza di std::io::Stdin
    // .read_line(&mut guess) ritorna un istanza di std::io::Result< || Invoca il metodo che legge una riga di input da tastiera>
    // &mut guess ritorna un istanza di std::io::Result<String>. Crea un errore perchè il compilatore vuole sapere dove và l'input che è pure modificabile.
    // .expect("Failed to read line") ritorna un istanza di std::io::Result. Gestisce, seppur male, un errore o problemi durante l'uso d stdin

    // !!!RUST PASSA PER RIFERIMENTO!!!

    println!("Hai inserito: {}", guess);
    // il messaggio in uscita è come c: metti un "blank" {} e poi inserisci la variabile

}

// di più su .expect("...") :

/*
    | std::io::Stdin::readl_line ritorna std::result::Result, per la precisione std::io::Result<usize>
    | in Rust non esistono eccezioni. I risultati (KO o OK) devono essere esplicitamente gestiti.
    | se ci dimentichiamo di gestirli, il programma non compila o il compilatore ci dice di farlo.
    | Result<T, E> è un tipo di dati che rappresenta un risultato di una operazione. Un enumerazione:
    | T è il risultato OK! quindi andata bene / Ok(T)
    | E è il risultato KO! quindi andata negativa/ Err(E)
    | .expect("...") è una funzione che restituisce il risultato di una operazione. In questo caso ritorna il numero di caratteri letti
    nell'esempio il valore viene ignorato, l'operazione fallisce e fa esplodere il programma (panic) con il messaggio indicato "failed to read line".
*/