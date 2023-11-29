pub struct NewsArticle
{
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle
{
    /*fn summarize(&self) -> String
    {
        format!("{}, by {}", self.headline, self.author)
    }*/
}

pub struct Tweet
{
    pub username:String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet
{
    fn summarize(&self) -> String
    {
        format!("{}: {}", self.username, self.content)
    }
}

// i treats/tratti, ci permettono di definire un set di metodi condivisi tra i file/strutture

pub trait Summary
{
    fn summarize(&self) -> String
    {
        String::from("(leggi di piÃ¹...)")
    }
}

// ----------------------------------------------------------------

fn main() 
{
    let tweet = Tweet
    {
        username: String::from("@Franco"),
        content: String::from("Salve Signori"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle
    {
        author: String::from("Gianni"),
        headline: String::from("Maledetta tecnologia"),
        content: String::from("Oggi Siri mi ha fatto arrabbiare"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());
}
