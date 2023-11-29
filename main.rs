/*struct Point 
{
    x: i32,
    y: i32,
}*/

/*struct Point<T>
{
    x: T,
    y: T,
}*/

struct Point<T, U>
{
    x: T,
    y: U,
}
//----------------------------------------------------------------
struct Poin<T>
{
    x: T,
    y: T,
}

impl<T> Poin<T> // questo metodo sarà disponibile per qualsiasi valore, poichè generico
{
    fn x(&self) -> &T
    {
        &self.x
    }
}

impl Poin<f64> // questo metodo/implementazione è disponibile solo per le variabili f64! Poichè non generico!
{
    fn y(&self) -> f64{
        self.y
    }
}
//----------------------------------------------------------------

struct Esem<T, U>
{
    x: T,
    y: U,
}

impl <T, U> Esem<T, U>
{
    fn mixup<V, W>(self, other: Esem<V, W>) -> Esem<T, W>
    {
        Esem
        {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // funzioni di estrazione

    let number_list = vec![34, 50, 25, 100, 65];

    /*
    let mut largest = number_list[0];

    for n in number_list
    {
        if n > largest
        {
            largest = n;
        }
    }
    */
 // se si seleziona il codice e si preme tasto destro, avremo l'opzione "refactor", che ci permette di creare istantaneamente una funzione

    let largest = piu_grande(number_list);

    println!("Il numero più grande è: {}", largest);

    let parola = vec!['c', 'i', 'a', 'o'];

    let largest = get_largest(parola);
    println!("La lettera più grande è: {}", largest);

    // ---------------------------------------------------
    // possiamo usare anche i tipi generici con le strutture.

    let p1 = Point { x: 5, y: 10 };
    //let p2 = Point {x:5.0, y:10.0}; Come fare? con i tipi generici!
    let p2 = Point {x:5.0, y:10.0};
    // let p3 = Point {x:5, y:10.0}; Ma questoi darà errore, Perchè SI, è un tipo generico, Ma NO! non sono dello stesso tipo genrico T!
    // come risolvere? aggiungiamo un altro tipo generico!
    let p3 = Point {x:5, y:10.0};

    // ---------------------------------------------------
    enum Option<T>
    {
        Some(T),
        None,
    }

    enum Result<T, E>
    {
        Ok(T),
        Err(E),
    }

    // li capite meglio ora questi no?
    // ---------------------------------------------------

    let p = Poin { x: 5, y: 10 };
    p.x();
    let p1 = Poin { x: 5.0, y: 1.0 };
    p1.x();
    p1.y();

    // Diversi esempi di cosa sia possibile fare nel video. (riga 41)
    let p1 = Esem{ x: 5, y: 10.4};
    let p2 = Esem{ x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

fn piu_grande(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];

    for n in number_list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

// ma se non vogliamo inserire solo vettori di interi?
//inseriamo il <T>, come per indicare un tipo generico e...
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for n in number_list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

// Solo che T è troppo generico per far funzionare gli operatori booleani. Quindi dobbiamo indicare un range di "Traits", dei "Tratti"
// (prossimo capitolo)
