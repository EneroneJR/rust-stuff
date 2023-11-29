struct Utente
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] 
struct Rectangle
{
    base: i32,
    altezza: i32,
}

impl Rectangle // con impl IMPLEMENTIAMO un oggetto, una struttura. in questo caso rettangolo. E dentro gli assegniamo tutti i metodi correlati.
{
    fn area(&self) -> u32
    {
        self.base * self.altezza
    }

    fn puo_contenere(&self, other: &Rectangle) -> bool // controlliamo se può contenere al suo interno un altro rettangolo più piccolo
    {
        self.base > other.base && self.altezza > other.altezza
    }
}

impl Rectangle
{
    fn square(size: u32) -> Rectangle
    {
        Rectangle
        {
            base: size,
            altezza: size,
        }
    }
}

fn main() {

    // Vediamo le diverse maniere per riempire una struttura dati

    // manualmente
    let mut utente1 = Utente{
        email: String::from("babbo69@gmail.com"),
        username: String::from("taralli"),
        active: true,
        sign_in_count: 1,
    };

    let nome = utente1.username;
    utente1.username = String::from("Tarallo"); // cambiare anche durante il programma

    // tramite funzioni 
    let utente2 = build_utente(String::from("gianfranco3000@gmail.com"), String::from("SuperGianfranco"));

    // Sfruttando un altra struttura esistente
    let utente3 = Utente
    {
        email: String::from("vetrugno@gmail.com"), // ricorda di mettere la virgola
        username: String::from("Vetrugnos"),
        ..utente2 // questo riempie gli altri campi, tramite la variabile già creata!
    };

    // Strutture TUPLE

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // come implementare le tuple?

    /*
    let altezza = 30;
    let larghezza = 50;

    println!("l'area del rettangolo è: {}", area_rettangolo(altezza, larghezza));
    */
    //corretto:

    let rettangolo = (30, 50);

    println!("l'area del rettangolo è: {}", area_rettangolo(rettangolo));

    let rect = Rectangle
    {
        altezza: 30,
        base: 50,
    };

    println!("l'area del rettangolo è: {}", struct_area_rettangolo(&rect));

    // println!("rettangolo: {}", rectangle); è possibile farlo? certo! Ma risolvendo alcuni errori...
    println!("rettangolo: {:#?}", rectangle); // MA NON BASTA! DOBBIAMO METTERE... #[derive(Debug)] sopra la dichiarazione della struttura!!!
    //si può fare :? o per renderlo più leggibile/ordinato :#?

    //----------------------------------------------------------------
    // Vediamo che cosa sono i Metodi... riga 16.

    println!("Area, ma con il metodo: {}",rect.area()); // abbiamo usato un metodo

    let rect1 = Rectangle
    {
        altezza: 15,
        base: 25,
    };
    let rect2 = Rectangle
    {
        altezza: 60,
        base: 100,
    };

    println!("Il rettangolo può contenere il rettangolo1: {}", rect.puo_contenere(&rect1));
    println!("Il rettangolo può contenere il rettangolo2: {}", rect.puo_contenere(&rect2));

    //----------------------------------------------------------------
    // funzioni associate: rigo 29

    let rect3 = Rectangle::square(25);
    
}    

fn area_rettangolo(dimensioni: (u32, u32)) -> u32 // in questo caso però, non sappiamo se è la base o altezza quella che viene passata. Ma si può risolvere facendo passare una struttura
{
    dimensioni.0 * dimensioni.1
}

fn struct_area_rettangolo(rectangle: &Rectangle) -> i32 // il &Rectangle, per non prendere ownership, ricorda
{
    rectangle.altezza * rectangle.base // molto più leggibile!
}
/*
fn area_rettangolo(altezza: i32, larghezza: i32) -> i32
{
    altezza*larghezza;
}
*/
fn build_utente(email: String, username: String) -> Utente
{
    Utente
    {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
