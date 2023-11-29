fn main() {
    let x: i32 = 6;
    println!("Il valore di x è {}", x);
    let x: i32 = 5;
    println!("il valore di x è {}", x);
    let x: &str = "Six";                    // LE VARIABILI POSSONO ESSERE RIDICHIARATE, CANCELLANDO QUELLE PRECEDENTI GIÀ DICHIARATE. SI CHIAMA "SHADOWING"
    println!("Il valore di x è: {}", x); 

    const ESEMPIO: u32 = 100000; // LE COSTANTI SEMPRE IN MAIUSCOLO

    const ESEMPIO2: u32 = 100_000; // I VALORI POSSONO ESSER SCRITTI ANCHE CON "_" PER AIUTARE LA LETTURA DEL NUMERO    

    //----------------------------------------------------------------

    //integers già visti... MA!
    
    let a: i32 = 98_222; // Input Decimale
    let b: i32 = 0xff; // Input Esadecimale
    let c: i32 = 0o77; // Input Ottale
    let d: i32 = 0b1111_0000; // Input Binario
    let e: u8 = b'A'; // Input a Byte(???) (supportato soltanto per u8)

    let f: u8 = 255; // massimo valore che si può inserire in un U8, poichè numero massimo per l'appunto di 2^8 = 256 (0..255), oltre crea OVERFLOW
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a volte questo in alcune versioni annulla il valore direttamente o direttamente sottrae 255 per evitare che il programma cada in panico
    
    //floating point numbers (Numeri con la virgola)

    let f: f64 = 2.0;
    let g: f32 = 3.0;

    // OPERAZIONI
    //addizioni
    let somma_variabile: i32 = 5 + 10;
    //sottrazioni
    let differenza: f64 = 95.5 - 4.3;
    //moltiplicazioni
    let prodotto: i32 = 4 * 30;
    //divisioni
    let quoziente: f64 = 56.7 / 32.2;
    //resto
    let resto: i32 = 43 % 5;

    //booleani

    let t: bool = true;
    let f: bool = false;

    //caratteri

    let c: char = 'z';
    let z: char = 'ℤ';
    let diamante: char = '♦';

    // FUNZIONI! LE TROVATE A RIGA: 128..154

    hello_moon();

    let s = somma(2, 5);

    // -----------------------------

    //controllo del "Flusso", la prima parte, giá fatto. (La parte relativa al controllo degli if e di assegnazione variabili tramite if). Ne riporterò comunque gli esempi.

    let numero: i32 = 5;

    if numero < 10
    {
        println!("La prima condizione è vera");
    }else if numero < 22
    {
        println!("Anche la seconda condizione è vera");
    }else 
    {
        println!("La condizione è falsa");
    }

    let condizione: bool = true;
    let numero: i32 = if condizione { 5 } else { 6 };

    //possiamo catturare eventuali valori con loop ed esistono 3 tipi di loop (iterazioni) in Rust
    let mut contatore: i32 = 0;

    /*loop 
    {
        contatore += 1;

        if contatore == 10
        {
            break contatore;
        }
    }*/

    let risultato:i32 = loop 
    {
        contatore += 1;

        if contatore == 10
        {
            break contatore;
        }
    }; // RICORDA DI USARE la semicolona però. 

    contatore = 3;

    while contatore != 0
    {
        println!("{}!", contatore);

        contatore -= 1;
    }

    println!("uscito dal while");

    let vettore:[i32; 5] = [10, 20, 30, 40, 50];

    for elemento in vettore.iter() // Non indicate il tipo, lo fa da sè altrimenti errore, non so al momento perchè.
    {
        println!("il valore è: {}", elemento);
    } 

    for num in 1..4
    {
        println!("{}!", num);
    }

}

//possiamo anche creare delle funzioni fuori dal main!

fn hello_moon() //funzioni scritte sempre in minuscolo
{
    println!("Salve Luna!");
}

// possiamo creare anche funzioni che prendono dentro valori:

fn somma_vuota(x: i32, y: i32)
{
    x+y;
} // ma ricordate che è un tipo VOID! NON RITORNA NULLA! Come risolvere?

fn somma(x: i32, y: i32) -> i32 //questo indica il "tipo" della funzione e cosa ritornerà il comando "return"
{
    let r: i32 = x + y;
    println!("La somma è: {}", r);
    // return r; POSSIAMO USARE RETURN, MA ANCHE NO! IN RUST È IMPLICITO CHE L'ULTIMA ESPRESSIONE SIA IL "RETURN"
    r // PERÒ DOBBIAMO OMETTERE IL ";" PER FARLO CAPIRE!!!
    // x=y      È COMUNQUE VALIDO!!!
}

fn somma_ottimizzata(x: i32, y: i32) -> i32
{
    x+y
}
