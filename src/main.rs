use std::env;

use client::Client;
use model::{LoginResult, Command};

mod cli_interaction;
mod communication;
mod model;
mod client;

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
            "Returns a list of matching Productcarriers, out of the result list one product can be selected to show the details of it.\n\
            Navigate the result list with arrow keys and select one result with space/enter".to_string(),
        ],
        short_description: "search <a query string>".to_string(),
    });

    client.start();
}
