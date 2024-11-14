use meetup_talk_ai_todo::{config::Config, run};

fn main() {
    let config = Config::new().unwrap();

    match run(config) {
        Ok(_) => println!("Thanks for using AI Todo, please come again."),
        Err(error) => eprintln!("There was an error using AI Todo :( {error}"),
    }
}
