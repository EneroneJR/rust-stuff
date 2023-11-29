use std::env;
// use std::fs; togliamo le librerie che intanto saranno integrate già in lib.rs
use std::process; // questo ci aiuta ad usare un comando utile come unwrap or else
// use std::error::Error; // insieriamo il valore Error

use minigrep::Config;

// se vogliamo scrivere gli errori su un file.txt, possiamo scrivere: Cargo run > output.txt
// attenzione che questo reinderizza in generale l'output sul file scelto!

fn main() 
{
    let args: Vec<String> = env::args().collect(); // args ci dà un iterazione, mentre collect in rende l'iterazione in una collezione
    // in questo caso, con "cargo run", inseriamo anche direttamente gli argomenti, esempio:
    // cargo run ciao mondo / cargo run test esempio.txt

    /*
    let query = &args[1]; // si mette uno perchè a noi non ci interesserà il path binario che viene restituito. Infatti il primo risultato del vecstring. è "target/debug/minigrep"
    let filename = &args[2];
    */
    //let (query, filename) = parse_config(&args);
    // let config = Config::new(&args); // ricorda di richiamare i metodi implementati dalla struttura con "::"
    let config = Config::new(&args)
    .unwrap_or_else(|err|
        {
            //println!("Problema a passare gli argomenti: {}", err);
            eprintln!("Problema a passare gli argomenti: {}", err); // queste permette anche con l' output settato su un file.txt, di farlo uscire sul terminale
            process::exit(1);
        }
    );
    println!("{:?}", args);
    //println!("Cercando per {}", query);
    //println!("Nel file: {}", filename);
    println!("Cercando per {}", config.query);
    println!("Nel file: {}", config.filename);

    //let contents = fs::read_to_string(filename)
    /*let contents = fs::read_to_string(config.filename)
        .expect("Errore a leggere il file");

    println!("Con il testo:\n{}", contents);*/
    //run(config); questo però non gestisce il caso di errore, quindi possiamo fare così...
    //if let Err(e) = run(config)
    if let Err(e) = minigrep::run(config)
    {
        println!("errore di applicazione: {}", e);
        process::exit(1);
    }

    // piccola nota, si cerca di tenere questo programma quanto più atomico possibile. Infatti le funzioni e tutto il resto, si cerca di tenerle in librerie in file lib.rs, per rendere il main quanto più corto possibile.
}

/* // Spostiamo tutto questo codice in lib.rs per più corto il codice.
fn run(config: Config) -> Result<(), Box<dyn Error>> //così aggiungiamo la possibilità di controllare eventuali errori
{
    let contents = fs::read_to_string(config.filename)?;
        //.expect("Errore a leggere il file");

    println!("Con il testo:\n{}", contents);
    Ok(())
}

struct Config
{
    query: String,
    filename: String,
}

impl Config
{
    /*
    fn new(args: &[String]) -> /*(&str, &str)*/ Config
    {
        if args.len() < 3
        {
            panic!("Inserire la richiesta e il nome del file");
        }
        let query = args[1].clone();
        let filename = args[2].clone(); // per evitare problemi con il borrowing

        //(query, filename)
        Config
        {
            query,
            filename,
        }
    }
    */ // sistemiamolo senza creare codice inutile:

    fn new(args: &[String]) -> /*(&str, &str) Config*/ Result<Config, &str>
    {
        if args.len() < 3
        {
            panic!("Inserire la richiesta e il nome del file");
        } // non mettete questa volta le ";", perchè è un possibile risultato!

        let query = args[1].clone();
        let filename = args[2].clone(); // per evitare problemi con il borrowing

        //(query, filename)
        Ok
        (
            Config
            {
                query,
                filename,
            }
        ) // l'errore uscirà da sè e verrà assegnato direttamente
    }

}

fn parse_config(args: &[String]) -> /*(&str, &str)*/ Config
{
    let query = &args[1].clone();
    let filename = &args[2].clone(); // per evitare problemi con il borrowing

    //(query, filename)
    Config
    {
        query, // se usi questo metodo, della struttura
        filename, // togli gli & all'argomento passato perchè usi il ".clone()"
    }
}
*/
