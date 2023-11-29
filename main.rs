use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

fn main() {

    // VETTORI
    let a = [1, 2, 3]; //ARRAY
    let mut v: Vec<i32> = Vec::new(); // Vettori (array dinamici)
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v2 = vec![1, 2, 3]; // è dinamico come a "v"
    }//uscito dal blocco, v2 ed i suoi elementi verranno eliminati

    let third = &mut v[2]; 
    println!("Il terzo elemento è {}", third);

    match v.get(20)
    {
        Some(third) => println!("il terzo elemento è {}", third),
        None => println!("Non c'è nessun elemento.")
    }

    enum SpreadsheetCell
    {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec!
    [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    match &row[1]
    {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Non un Intero!"),
    };

    // STRINGHE
    // le stringhe sono contenute in una collezione di bit codificati in UTF-8

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    // foobar!

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    //let s3: String = s1 + &s2; // qua togliamo l'ownership di s1, quindi ci sarà un errore in s1 poichè ora è vuoto!
    //println!("{}", s1);

    let s3 = format!("{}{}", s1, s2); // la macro format non prende ownership

    let hello: String = String::from("Hello");
    //let c: char = hello[0]; //questo non può andare bene, perchè UTF-8, 
    // porta i caratteri a poter arrivare ad essere lunghi fino a 4 bit!,
    // ad esempio la prima lettera di "Ciao" in russo è di 3 bit lunga! Per un solo carattere!

    // nell'utf-8 esistono 3 tipi di valori:

    // bytes

    println!("Bytes");

    for b in "Ciao".bytes() // non ho una tastiera speciale, ma metteteci qualsiasi parola volete voi, purtroppo i caratteri speciali dei scalari non li ho
    {
        println!("{}", b);
    }

    // scalari

    println!("scalari");

    for c in "Ciao".chars()
    {
        println!("{}", c);
    }

    // grafici

    println!("grafici / 'graphemes' ");

    // Qui non si può fare di norma! Bisogna importare una dependencies in cargo.toml e aggiungendolo (riga 1)

    for g in "Ciao".graphemes(true)
    {
        println!("{}", g);
    }

    // HASHMAPS

    // per usare gli hashmaps ci tocca prima aggiungere dalla libreria std (rigo 2)    

    let blue = String::from("Blue");
    let yellow = String::from("Yellow"); 

    let mut scores = HashMap::new();

    scores.insert( blue, 10); // ricorda che l'ownership viene passata
    scores.insert( yellow, 50); // quindi...
    // println!("{}", blue); non è valido!

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // in questo caso abbiamo un option, perchè non possiamo garantire che un valore sarà ritornato!

    for (key, value) in &scores
    {
        println!("{}: {}", key, value);
    }

    let mut punteggi = HashMap::new();

    punteggi.insert(String::from("Blue"), 10);
    punteggi.insert(String::from("Blue"), 20); //Così SOVRASCRIVIAMO il dato precedente con la chiave 'Blue'

    // come possiamo evitarlo?

    punteggi.entry(String::from("Yellow")).or_insert(30); // se non c'è un valore, o una chiave già esistente con questa chiave, creala ed inserisci 30
    punteggi.entry(String::from("Yellow")).or_insert(40); // se esiste, non fare niente

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // ["hello", "world", "wonderful", "world",]
    for word in text.split_whitespace()
    {
        let count = map.entry(word).or_insert(0); // in questo caso riporta anche un riferimento alla variabile
        *count += 1; // con * togliamo il riferimento alla variabile ed aggiungiamo un 1 al valore
    }

    println!("{:?}",map);
    
}
