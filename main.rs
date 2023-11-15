use std::{
    cmp::Ordering,
    io,
    io::Write, // nota che usi dentro le {}, la "," (virgola), non il punto e virgola.
}; // ricordarti sempre di finire con un ";"
use rand::Rng;

fn main() 
{
    println!("Generando il numero da indovinare...");
    let mut rng = rand::thread_rng(); // genera una variabile di tipo Rng che si occupa di generare un numero casuale richiamandola con .gen_range("0..N-1")
    let n = rng.gen_range(1..101); // richiamo la variabilke rng con il metodo .gen_range("1..101") che genera un numero casuale tra 1 a 100

    //let mut input = String::new(); // stringa vuota da riempire con l'input	da indovinare. ~~proviamo senza prendere in input una stringa~~
    // fare una variabile in input qui è superfluo visto che serve dentro al loop

    // ATTENZIONE! FARE QUESTO CREA ERRORE DURANTE IL CODICE! QUELLO SOPRA! LET MUT INPUT = String::new();!!!. I CODICI APPUNTATI SENZA SPIEGAZIONE SONO CIÒ CHE CAUSAVA L'ERRORE!

    println!("per motivi di test il valore generato è: {}", n);
    loop //loop porta ad un iteerazione che puo' uscire solo se ci sarà un break. Per questo è un iterazione non consigliata
    {
        print!("Inserisci un numero tra 1 e 100: ");
        io::stdout().flush().unwrap();


        let mut input: String= String::new(); // BISOGNA ASSOLUTAMENTE RIPORTARE LA VARIABILE DA INTERO A STRINGA!!
        // ALTRIMENTI IL PROGRAMMA NON PUÒ CONVERTIRE INTERI IN INTERI!! SOPRATTUTTO SE È SISTEMATO PER STRINGA!!


        io::stdin().read_line(&mut input).unwrap(); 

        let mut input: i32 = input.trim().parse().expect("Errore nella lettura"); // convertiamo da Stringa a intero

        println!("");
        //let input: u8 = input.trim().parse::<u8>().expect("inserisci un tipo di numero!"); // QUESTO CREA ERRORE SE ITERATO

        match input.cmp(&n) // Devi SEMPRE mettere tutti e 3 i casi.
        {
            Ordering::Less => println!("Troppo piccolo"),
            Ordering::Greater => println!("Troppo grande"),
            Ordering::Equal => { //Si. Puoi usare una sequenza di istruzione. la funzione match è più comoda di fare diversi "If".
                println!("Uguale! hai vinto!");
                break; // NON DIMENTICARE MAI UN ESCAPE CON BREAK!
            }
        }
    }

    // loop { <blocco> }, while <condizione> { <blocco> }, for <variabile> in <iteratore> { <blocco> }. 

    /*
        for i in 0..10 {
            println!("i = {}", i);
        }
    */

    println!("Ecco diversi test di conferma di iterazioni: /n for i in 0..10, vediamo:");

    
    let mut i = 0;

    for i in 0..10 { //nota, possiamo anche indicare il tipo di intero e obbligarlo con i:i32.
        println!("i = {}", i);
    } // va da 0 a 9. Quindi da N(0) a N(i)-1

    println!("while...");

    while  i < 10 
    {
        println!("i = {}", i);
        i += 1;
    } // va da 0 a 9. Quindi da N(0) a N(i)-1

    /*println!("Non stava nemmeno bisogno di usare una nuova variabile e parte da sola anche da 0 senza istanziarla! ora testiamo il do while...");

    do{
        println!("n = {}", n);
        n += 1;
    } while n < 10; */ //il do while non esiste in Rust
    /* per fare un  Do While, ci tocca ricorrere a fare:
        
        while(true)
        {
            expression
            if condition
            {
                break;
            }
        }

        oppure...

        loop 
        {
            do_stuff();
            if !condition 
            {
                break;
            }
        }

        o ancora... (da testare sia chiaro)

        while 
        {
            do_stuff();
            condition
        } {}

        quindi probabilmente in questi usare delle funzioni ecc. e quindi più affidabile la prima situazione e accettabile la seconda.
    */

    // per gli if la storia si complica. Rust accetta solo booleani.

    if (1<2)
    {   
        println!("1<2. Le parentesi non sono obbligatorie");
    }else
    {
        println!("1>2");
    }
    let o = 2;
    if o<2
    {   
        println!("1<2");
    }else
    {
        println!("1>2... sì, ho copia-incollato per fare un if sbagliato e controllare se si potevano fare i controlli normali.");
    }

    // risolto il capitolo sulle iterazioni, ricorda solo che If vuole booleani.

}
