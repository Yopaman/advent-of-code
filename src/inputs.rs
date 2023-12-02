use dotenv::dotenv;
use reqwest::blocking::Client;

fn get_url(day: &str) -> String {
    format!("https://adventofcode.com/2023/day/{}/input", day)
}

pub fn get_input(day: &str) -> String {
    dotenv().ok();
    let session = match std::env::var("session") {
        Ok(var) => var,
        Err(_) => panic!("Error : No session key provided."),
    };
    let client = Client::new();
    match client
        .get(get_url(day))
        .header("Cookie", format!("session={}", session))
        .send()
    {
        Ok(resp) => resp.text().unwrap(),
        Err(e) => panic!("{}", e),
    }
}
