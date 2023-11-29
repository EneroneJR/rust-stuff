use std::fs::File; // con questo permettiamo l'apertura dei filesystem
use std::io::ErrorKind;	// con questo possiamo gestire gli errori
use std::io::Read;
use std::error::Error;

fn main() {
    // per chiudere il programma con valori errati che non vogliamo, possiamo usare la macro:
    // panic!("Programma esploso perchè è stato richiamato dal programmatore");
    // esempio:

    //a();

    let f = File::open("hello.txt");

    let f = match f
    {
        Ok(file) => file,
        //Err(error) => panic!("Problema ad aprire il file: {:?}", error),
        Err(error) => match error.kind()
        {
           ErrorKind::NotFound => match File::create("Hello.txt") // creare un altro file POTREBBE fallire, quindi dobbiamo controllare che vada per bene!
           {
                Ok(fc) => fc,
                Err(e) => panic!("Problema a creare il file: {:?}", e),
           },
           other_error =>
           {
                panic!("Problema ad aprire il file: {:?}", other_error)
           }
        }
    };
    //----------------------------------------------------------------
    // per semplificare il tutto, possiamo usare unwrap, invece di match:

    //let f = File::open("Hello2.txt").unwrap(); //in questo caso farà tutto il codice di match, nel caso Ok prenderà il file, in quello di Err, darà panic!

    // ma abbiamo anche il metodo expect(). Che ci permette di specificare il messaggio di errore!
    let mut f = File::open("Hello2.txt").expect("Fallito ad aprire Hello2.txt");

    let mut s = String::new();

    f.read_to_string(&mut s); // ci evita tutto il lavoro del match
    Ok(s);

}

fn a()
{
    b(22);
}

fn b(x: i32)
{
    if x == 22
    {
        panic!("Non pasare il valore 22");
    }
}

fn read_from_file() -> Result<String, io::Error>
{
    Fs::read_to_string("HelloP.txt"); //fs sta per FileSystem
    Ok(())
}
