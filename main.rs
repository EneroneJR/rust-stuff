/* se nel use ci servono più rami/path di librerie. come Input e Output, ma anche "Ordering" possiamo fare...

    use rand::Rng;
    use std::io;
    use std::cmp::Ordering;

    Ma personalmente preferirei:
*/
use std::{
    cmp::Ordering,
    io, // nota che usi dentro le {}, la "," (virgola), non il punto e virgola.
}; // ricordarti sempre di finire con un ";"
use rand::Rng;

// Ordering è un enumerazione che contiene solo questi 3 valori:
/*
enum Ordering {
    Less,
    Equal,
    Greater,
} minore, uguale e maggiore

*/
fn main() 
{
    let mut rng = rand::thread_rng(); // genera una variabile di tipo Rng che si occupa di generare un numero casuale richiamandola con .gen_range("0..N-1")
    let n = rng.gen_range(1..101); // richiamo la variabilke rng con il metodo .gen_range("1..101") che genera un numero casuale tra 1 a 100
    let mut input = String::new(); // stringa vuota da riempire con l'input	da indovinare
    io::stdin().read_line(&mut input).unwrap();

    // con ordering abbiamo la funzione "match"
   /*match input.cmp(&n) { // cmp è un abbreviazione di "compare"/"comparare"
        Ordering::Less => println!("Troppo piccolo"),
        Ordering::Equal => println!("Uguale! hai vinto!"),
        Ordering::Greater => println!("Troppo grande"),
    }*/
    // problema: input è una stringa e non un numero. Mentre rng è un numero

    // come risolvere?
    // Rust è capace di comparare non solo gli interi, ma TUTTI gli oggetti che hanno determinati caratteristiche! 
    // Questo attraverso le "trait", ma questo si approfondisce su un altro programma.

    /* Rust ha diversi tipi di intero:
     i8 ovvero intero col segno a 8 bit: -128 a 127 ( da i8::Min a i8::Max ) SFRUTTA IL COMPLEMENTO A 2!
     u8 intero senza segno a 8 bit: 0 a 255 ( da 0 a u8::MAX)
     i16 intero col segno a 16 bit: -32768 a 32767 ( da i16::MIN a i16::MAX )
     u16 intero senza segno a 16 bit: 0 a 65535 ( da 0 a u16::MAX)
     i32 intero col segno a 32 bit: -2147483648 a 2147483647 ( da i32::MIN a i32::MAX )
     u32 intero senza segno a 32 bit: 0 a 4294967295 ( da 0 a u32::MAX)
     i64 intero col segno a 64 bit: -9223372036854775808 a 9223372036854775807 ( da i64::MIN a i64::MAX )
     u64 intero senza segno a 64 bit: 0 a 18446744073709551615 ( da 0 a u64::MAX)
     isize intero con segno, stessa dimensione dei puntatori per l'architettura corrente (usati magari per array)
     usize intero senza segno, stessa dimensione dei puntatori per l'architettura corrente (usati magari per displacement di array)
    */

    // detto questo, cambiamo la stringa ad intero no?
    // conversione:

    //let input: u32 = input; MA POTREBBE DARE ERRORE! Quindi usiamo le specifiche buone come .trim(), .parse() e .expect("inserisci un tipo di numero!")
    // per un porse() buono, possiamo fare .parse::<u32>() così che riporti non qualsiasi cosa, ma un tipo a 32 bit.
    let input: u8 = input.trim().parse::<u8>().expect("inserisci un tipo di numero!");

    /*
        Quindi in rust è completamente legale ridefinire variabili. Completamente:
        //...
        let guess = io::read_line().expect("...");
        let guess: u32 = guess.trim().parse().expect("...");
        //...
    */
    match input.cmp(&n) 
    {
        Ordering::Less => println!("Troppo piccolo"),
        Ordering::Equal => println!("Uguale! hai vinto!"),
        Ordering::Greater => println!("Troppo grande"),
    }
}
