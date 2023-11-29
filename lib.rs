use std::fs;
use std::error::Error;
// copiamo anche questi perchè altrimenti ci darà errore per alcune cose, come il ? simbolo o Error, env ecc.
use std::env;

// RICORDATE ANCHE CHE PER DEFAULT TUTTO IN RUST È PRIVATO! QUINDI SE VOGLIAMO USARLO DOBBIAMO RENDERLO PUBBLICO!

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive
    {
        search(&config.query, &contents)
    }else
    {
        search_case_insensitive(&config.query, &contents)
    };

    //println!("Con il testo:\n{}", contents);
    //for line in search(&config.query, &contents)
    for line in results
    {
        println!("{}", line)
    }
    Ok(())
}

pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config
{
    pub fn new(args: &[String]) -> Result<Config, &str>
    {
        if args.len() < 3
        {
            panic!("Inserire la richiesta e il nome del file");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSESITIVE").is_err(); // Questo permette di scriverci se vero o falso passando le variabili
        // in questo caso per settare possiamo fare così: export CASE_INSESITIVE=true, per settarlo come true
        // per toglierlo basta fare: unset CASE_INSENSITIVE

        Ok
        (
            Config
            {
                query,
                filename,
                case_sensitive,
            }
        )
    }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> // questa ricerca è case sensitive
{
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.contains(query)
        {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines()
    {
        if line.to_lowercase().contains(&query)
        {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn un_risultato()
    {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive()
    {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
