use bb_ollama::models::message::Message;
use meetup_talk_ai_todo::run;

fn main() {
    match run() {
        Ok(Message { content, .. }) => println!("{content}"),
        Err(error) => eprintln!("There was an error using AI Todo :( {error}"),
    }
}
