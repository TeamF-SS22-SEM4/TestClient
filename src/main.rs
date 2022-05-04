use communication::LoginResult;

mod cli_interaction;
mod communication;

struct Client {
    host_url: String,
    login_result: Option<LoginResult>,
}

impl Client {
    fn new(url: &str) -> Client {
        Client {
            host_url: url.to_string(),
            login_result: None,
        }
    }

    fn login(&mut self, username: &str, password: &str) -> () {
        self.login_result = communication::login(&self.host_url, username, password);
    }

    fn start(&mut self) -> () {
        println!("Starting client for {}", self.host_url);

        let (username, password) = cli_interaction::get_credentials();

        self.login(&username, &password);

        println!("{:#?}", self.login_result);
    }
}

fn main() {
    let mut client = Client::new("http://localhost:8080/api/v1");

    client.start();
}
