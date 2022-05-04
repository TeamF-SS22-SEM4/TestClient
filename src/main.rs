use communication::LoginResult;
use communication::Reason;

mod cli_interaction;
mod communication;

struct Client {
    host_url: String,
    session_info: Option<LoginResult>,
}

impl Client {
    fn new(url: &str) -> Client {
        Client {
            host_url: url.to_string(),
            session_info: None,
        }
    }

    fn login(&mut self) -> () {
        let mut logging_in = true;

        while logging_in {
            let (username, password) = cli_interaction::get_credentials();

            self.session_info = match communication::login(&self.host_url, &username, &password) {
                Ok(session_info) => {
                    logging_in = false;
                    Some(session_info)
                }
                Err(reason) => {
                    match reason {
                        Reason::UNAUTHORIZED => {
                            logging_in = cli_interaction::retry_credentials();
                        }
                        Reason::BadRequest => {
                            panic!("Internal error! Bad Request while logging in");
                        }
                        Reason::OTHER => {
                            panic!("Unknown error while logging in. {:?}", reason);
                        }
                    };
                    None
                }
            };
        }
    }

    fn start(&mut self) -> () {
        println!("Starting client for {}", self.host_url);

        self.login();

        println!("{:#?}", self.session_info);
    }
}

fn main() {
    let mut client = Client::new("http://localhost:8080/api/v1");

    client.start();
}
