fn main() 
{
    let mut s = String::from("ciao mondo");

    let ciao = &s[0..4];
    let mondo = &s[5..10]; // così abbiamo fatto uno slice, ma come implentarlo per una funzione? ATTENZIONE! Darà errore

    let s2 = "ciao mondo";

    let word = first_word(s2);

    //let parola = first_word(&s);
    s.clear(); // il comando clear cancella la stringa

    // possiamo avere slice anche su array

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
}

fn first_word(/*&s: &String*/ s: &str) -> &str
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[0..i];
        }
    }

    &s[..]
}

/*fn first_word(&s: &String) -> usize
{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }

    s.len()
}
*/
