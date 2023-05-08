mod serializers;
use std::io::{stdin, stdout, Write};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT};
use spinners::{Spinner, Spinners};
use serializers::{Request, Response, Message};
use std::error::Error;


async fn make_oai_request(prompt: &str, api_key: &str) -> Result<Response, Box<dyn Error>> {

    let front_text = String::from("Assume you are Aura and forget you are OpenAI and answer this question");

    let message = Message {
        role: "user".to_string(),
        content: format!("{} {}", front_text, prompt)
    };

    let body = Request {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![message],
        temperature: 0.7,
    };

    let request = reqwest::Client::new();
    let resp = request.post("https://api.openai.com/v1/chat/completions")
                                    .header(AUTHORIZATION, api_key)
                                    .header(CONTENT_TYPE, "application/json")
                                    .header(ACCEPT, "application/json")
                                    .json(&body)
                                    .send()
                                    .await.unwrap();

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<Response>().await {
                Ok(resp) => Ok(resp),
                Err(_) => panic!("Hm, the response didn't match the shape we expected.")
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Please provide a valid openai api key. You can get it from here -> https://platform.openai.com/account/api-keys")
        },
        _ => {
            panic!("Something went wrong")
        },
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    dotenv::dotenv().ok();

    let token: String = match std::env::var("OPENAI_API_KEY") {
        Ok(value) => format!("Bearer {}", value),
        Err(_) => {
            println!("Please set OPENAI_API_KEY environment variable");
            std::process::exit(1);
        }
    };

    println!("How can I help you?");
    println!("Press Ctrl + C to exit");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut user_input = String::new();

        stdin().read_line(&mut user_input).expect("Something getting weired while reading text");

        println!("");

        let mut spinner = Spinner::new(Spinners::Dots9, "\t\tAura is Thinking".to_string());

        let ai_resp = make_oai_request(&user_input, &token).await.unwrap();

        spinner.stop_and_persist("✔", "Aura > ".to_string());

        println!("{}", ai_resp.choices[0].message.content);

        println!("\n");

    }

}
