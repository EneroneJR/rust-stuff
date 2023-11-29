mod back_of_house
{
    pub struct Breakfast
    {
        pub toast: String,
        frutta_stagionale: String,
    }

    impl Breakfast
    {
         pub fn summer(toast: &str) -> Breakfast
        {
            Breakfast 
            { 
                toast: String::from(toast),
                frutta_stagionale: String::from("Pesche"),
            }
        }
    }
}

/*mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {}
    }
}

pub use crate::front_of_house::hosting; // ma senza pub non ci permetterà di usare la funzione per altre parti o codici!
*/

mod front_of_house; // usare questa notazione, aiuta rust a capire che sta un altro file parallelo da cui usare il modulo

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant()
{
    let mut meal = back_of_house::Breakfast::summer("Rye"); // Se Struct Breakfast non è Pubblica, darà errore. Così come la funzione 

    meal.toast = String::from("Wheat"); // seppur toast, sia dentro una struttura pubblica... I campi rimangono privati. Quindi Pub finisce anche dietro ad i campi
    // ATTENZIONE! SOLO LE STRUTTURA AVRANNO I CAMPI PRIVATI! GLI ENUMERATORI TERRANNO PUBBLICO TUTTO IL RESTO!
}

/*mod front_of_house_
{
    pub mod hosting
    {
        pub fn add_to_waitlist()
        {}
    }
}

pub fn eat_at_restaurant()
{
    // path assoluto
    crate::front_of_house_::hosting::add_to_waitlist();

    // path relativo
    front_of_house_::hosting::add_to_waitlist();
}

fn serve_order()
{}

mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();
        super::serve_order();
    }

    fn cook_order()
    {}
}*/

/*mod front_of_house
{
    mod hosting
    {
        fn add_to_waitlist()
        {}

        fn seat_at_table()
        {}
    }

    mod serving
    {
        fn take_order()
        {}

        fn serve_order()
        {}

        fn take_payment()
        {}
    }
}*/

/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
