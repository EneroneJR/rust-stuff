/*enum IpAddr
{
    V4,
    V6,
}*/

enum IpAddr
{
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}

// possiamo anche sfruttare un enumeratore per tenere più strutture dati o fare di più!

enum Message
{
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Possiamo anche inserirci metodi!
impl Message
{
    fn funzione()
    {
        println!("Questo è un esempio");
    }
}

struct IpAddress
{
    kind: IpAddr,
    address: String,
}

fn main() 
{
    /* Esempio poco pratico di cosa potrebbe fare un Enumeratore
    let quattro = IpAddr::V4;
    let sei = ipAddr::V6;

    let localhost = IpAddress
    {
        kind: IpAddr::V4,
        address: String::from("127.0.0.1"),
    };
    */ // Sistemiamo

    // let localhost = IpAddr::V4(String::from("127.0.0.1"));

    let localhost = IpAddr::V4(127, 0, 0, 1);

    // in Rust non esiste il valore NULL, ma possiamo fare "l'option enum", Enum opzionale:
/*    enum Option<T>
    {
        Some(T),
        None,
    }
*/ // Questa cosa è così importante che Rust lo ha implentato già da sè!

    let qualche_numero =Some(5);
    let qualche_stringa = Some("Una stringa");

    let numero_assente: Option<i32> = None; // nota come nel caso di Some, non abbiamo dovuto implementare il tipo!!!

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum: <i8 as Add<Option<i8>>>::Outpot = x + y; è un errore perchè i tipi sono diversi, quindi dobbiamo estrarne la variabile.
    // però e specifico per l'option usata
    let sum = x + y.unwrap_or(0);


}

fn route(ip_kind: IpAddress)
{
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valore_in_centesimi(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny => {
            println!("Penny fortunato!");
            50;
        },  //esempio
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
