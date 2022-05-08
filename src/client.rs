use crate::cli_interaction;
use crate::communication;

use crate::model::{Command, LoginResult, Reason};

pub struct Client {
    pub host_url: String,
    pub session_info: Option<LoginResult>,
    pub commands: Vec<Command>,
}

impl Client {
    pub fn new(url: &str) -> Client {
        Client {
            host_url: url.to_string(),
            session_info: None,
            commands: vec![],
        }
    }

    pub fn login(&mut self) -> () {
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

    pub fn start(&mut self) -> () {
        println!("Starting client for {}", self.host_url);
        self.login();

        if self.session_info.is_some() {
            println!("Ready to receive commands type. Type \"help\" for a list of commands ");
            self.receive_commands()
        } else {
            self.shutdown()
        }
    }

    pub fn add_command(&mut self, command: Command) -> () {
        self.commands.push(command)
    }

    pub fn shutdown(&self) {
        println!("Shutting down.")
    }

    pub fn receive_commands(&mut self) -> () {
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
