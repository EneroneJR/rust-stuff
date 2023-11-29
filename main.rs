fn main() {
    // Capitolo sulle Ownership. Un argomento per esempio sono lo Stack e l'Heap
    /*
    fn main() 
    {
        fn a()
        {
            let x: &str = "Hello";
            let y: i32 = 22;
            b();
        }
        fn b()
        {
            let x: String = String::from("world");
        }
    }
    */
    //--------------------------------------------------------
    // ----- regole di Ownership ------
    // 1. Ogni valore in Rust ha una variabile chiamata Proprietario.
    // 2. Ci può essere solo UN proprietario alla volta.
    // 3. Quando il Proprietario esce fuori dal "focus", portata, il valore viene "Lasciato", cancellato.

    { // s non è valido qui, non è ancora dichiarata
        let s: &str = "Hello"; // s è valida da adesso in avanti
        // scrivi codice con s
    } // adesso usciamo dal focus, finendolo ed s non è più valido

    /*  s nell'esempio però è di tipo finito (ricordare che la stringa è un array di char). Per farla dinamica possiamo fare:
        let s: String = String::from("Hello");   che è come allocare una memoria dinamica con la funzione "new"
        quando usciamo dal "focus", viene in automatico/sottinteso che avviene un "delete"
    */

    let x: i32 = 5;
    let y: i32 = x; //Copia

    let s1: String = String::from("Hello"); // Le stringhe dinamiche sono come un puntatore. Quindi s1, PUNTA la memoria/array dinamico con la stringa.
    // Quindi s2 copia l'indirizzo di memoria?
    let s2: String = s1; // Muove, non copia. da adesso. s1 non avrà più il valore o il puntatore che Punta verso la memoria dove è allocata la stringa.
    // come risolvere?
    // let s2: String = s1.clone();

    // Ora. Ogni elemento semplice, dentro lo stack, viene COPIATO non MOSSO. Mentre gli elementi dentro l'heap (che si accede tramite puntatore)...
    //...Viene MOSSO non COPIATO.

    let s: String = String::from("hello");
    prendi_ownership(s); // qui viene presa la proprietà del valore (in origine la funzione si chiamava take_ownership)...
    //println!("{}", s);                 generando un errore qua, poichè s non ha più niente

    let x: i32 = 5;
    crea_copia(x); // qua viene creata una copia del valore, senza creare errori
    println!("{}", x);

    // ma può succedere anche l'opposto! (le funzioni sopra erano dei comuni print)

    let s1: String = lascia_ownership(); // (in origine la funzione si chiamava gives_ownership) 
    println!("s1 = {}", s1);
    
    let s1 = lascia_ownership();
    let s2 = String::from("hello");
    let s3 = prendi_e_lascia_indietro(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    // e se vogliamo usare una variabile senza prenderne l'ownership? Si usano le Reference
    let s1 = String::from("Basta Hello");
    let (s2, len) = calcola_lunghezza(s1);
    println!("La lunghezza di '{}' è {}.", s2, len);

    /*possiamo risolvere una funzione così strana facendo:
        let len = calcola_lunghezza(s1);
        println!("La lunghezza di '{}' è {}.", s1, len);

        fn calcola_lunghezza(s: String) -> usize
        {
            let lunghezza = s.len(); // len() ritorna la lunghezza di una stringa
            lunghezza
        }

        MA DARÀ ERRORE. Perchè!? Perchè s1, avrà perso la sua ownership!!!

        come risolvere? tramite "&".

        fn main()
        {
            let s1 = String::from("Basta Hello");
            let len = calcola_lunghezza(&s1);  ------------------metti la & prima di passare la variabile
            println!("La lunghezza di '{}' è {}.", s1, len);
        }
        fn calcola_lunghezza(s: &String) -> usize ---------------metti la & prima di passare il tipo 
        {
            let lunghezza = s.len(); // len() ritorna la lunghezza di una stringa
            lunghezza
        }
        
        Questo si chiama "Prestito", perchè finita la funzione, l'ownership da parte della funzione cessa. 
        Inoltre ricordate che è come si creasse un puntatore s che punta alla stringa s1 che punta all'effettiva allocazione di memoria della stringa
        Quindi s, come s1, è IMMUTABILE, non cambiabile!!!

        Però! Se volete passare un riferimento MUTABILE il discorso CAMBIA!!!
        let mut s1 ~~~~~~;
        cambia(&mut s1);

        fn cambia(s: &mut String)
        {~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~}

        ATTENZIONE, non possiamo fare più di un prestito alla volta Se la variabile o il prestito è mutabile!
        1) prestiti immutabili si possono fare quante volte vogliamo. ANCHE CON VARIABILI MUTABILI
        let mut s1;
        s2 = &s1;
        s3 = &s1;
        2) prestiti mutabili sono errori se prima stava qualsiasi tipo di prestito
        s4 = &mut s1: <-----ERRORE, prima stanno altri due prestiti
        3) una volta finiti però i prestiti, possiamo chiedere tranquillamente un prestito mutabile, SE PRIMA NON CE NE SONO ALTRI ATTIVI

        REGOLE DI RIFERIMENTO
        1) Ad ogni tempo, puoi avere o un riferimento mutabile OPPURE ogni numero di riferimenti immutabili

        2) i riferimenti devono sempre esser validi
    */

    //esistono anche gli slicesm, ma questo sarà per un altro capitolo
}

fn calcola_lunghezza(s: String) -> (String, usize) 
{
    let lunghezza = s.len(); // len() ritorna la lunghezza di una stringa
    (s, lunghezza)
}

fn lascia_ownership() -> String
{
    let una_stringa: String = String::from("Hello");

    una_stringa
}

fn prendi_ownership(una_stringa: String)
{
    println!("{}", una_stringa);
}

fn prendi_e_lascia_indietro(una_stringa: String) -> String
{
    una_stringa
}

fn crea_copia(un_intero: i32)
{
    println!("{}", un_intero);
}
