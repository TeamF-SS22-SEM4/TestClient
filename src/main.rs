use std::env;

use model::{Reason, LoginResult};

mod cli_interaction;
mod communication;
mod model;

struct Client {
    host_url: String,
    session_info: Option<LoginResult>,
    commands: Vec<Command>,
}

impl Client {
    fn new(url: &str) -> Client {
        Client {
            host_url: url.to_string(),
            session_info: None,
            commands: vec![],
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
                        Reason::InvalidCredentials => {
                            logging_in = cli_interaction::retry_credentials();
                        }
                        Reason::Other => {
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

        if self.session_info.is_some() {
            println!("Ready to receive commands type. Type \"help\" for a list of commands ");
            self.receive_commands()
        } else {
            self.shutdown()
        }
    }

    fn add_command(&mut self, command: Command) -> () {
        self.commands.push(command)
    }

    fn shutdown(&self) {
        println!("Shutting down.")
    }

    fn receive_commands(&mut self) -> () {
        let mut receiving = true;

        while receiving {
            let args = cli_interaction::get_command();
            let command_name = &args[0];
            let mut handled_input = false;

            //meta commandds are defined here and not via Command-struct
            if command_name.eq_ignore_ascii_case("quit") {
                receiving = false;
                handled_input = true;
            }
            //TODO help-command
            if command_name.eq_ignore_ascii_case("help") {
                println!("Command: ");
                println!("help   - displays this list\n-----------------");

                println!("Command: ");
                println!("quit   - exits the program\n-----------------");

                for c in &self.commands {
                    println!("Command: ");
                    c.print_help();
                    println!("-----------------")
                }
                handled_input = true;
            }

            for c in &self.commands {
                if command_name.eq_ignore_ascii_case(&c.name) {
                    (c.action)(&self, &args[1..]);
                    handled_input = true;
                }
            }

            if !handled_input {
                println!("Unknown command");
            }
        }
    }
}

struct Command {
    name: String,
    action: fn(calling_client: &Client, args: &[String]) -> (),
    arg_descriptions: Vec<String>,
    short_description: String,
}
impl Command {
    fn print_help(&self) {
        println!("{}", self.short_description);
        for arg in &self.arg_descriptions {
            println!("{}", arg);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut url = "http://localhost:8080/api/v1";
    if args.contains(&"remote".to_string()) {
        url = "http://10.0.40.170:8080/api/v1";
    }

    let mut client = Client::new(url);
    client.add_command(Command {
        name: "search".to_string(),
        action: |calling_client: &Client, args| {
            let mut query = String::new();
            for a in args {
                query.push_str(&a);
                query.push(' ');
            }
            let products = communication::search(&calling_client.host_url, &calling_client.session_info.as_ref().unwrap_or(&LoginResult::new()).session_id, &query);

            if products.len() == 0 {
                println!("Seems like no products matched your search...");
            } else {
                let selection = cli_interaction::select_product(&products);

                if let Ok(Some(index)) = selection {
                    let product_id = &products.get(index).expect("Vector's be drinking and loosing items unexpectedly").id;
                    
                    if let Some(product) = communication::get_product(&calling_client.host_url, &calling_client.session_info.as_ref().unwrap_or(&LoginResult::new()).session_id, product_id) {
                        println!("{}", product);
                    }
                }
            }
        },
        arg_descriptions: vec![
            "query   - search this text in productcarriers. Do not wrap the query with \" "
                .to_string(),
            "Returns a list of matching Productcarriers, out of the result list one product can be selected to show the details of it.".to_string(),
        ],
        short_description: "search <a query string>".to_string(),
    });

    client.start();
}
