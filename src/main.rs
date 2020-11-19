use std::env::args;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time;

use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

use discord_rpc_client::Client;

// discord_rpc filename
fn main() {
    let filename = args().nth(1).unwrap();
    let mut config = load_config(&filename);

    let client_id = config["client_id"].as_i64().unwrap() as u64;
    let mut drpc = Client::new(client_id);
    drpc.start();
    println!("Started DRPC");

    let interval_millis = config["interval"].as_i64().unwrap_or(15000) as u64;
    let interval = time::Duration::from_millis(interval_millis);
    loop {
        config = load_config(&filename);
        if let Err(e) = drpc.set_activity(|mut a| {
            if config["details"].as_str().is_some() {
                a = a.details(config["details"].as_str().unwrap());
            }
            if config["state"].as_str().is_some() {
                a = a.state(config["state"].as_str().unwrap());
            }
            a.assets(|mut asts| {
                if config["image"].as_str().is_some() {
                    asts = asts.large_image(config["image"].as_str().unwrap());
                }
                if config["tooltip"].as_str().is_some() {
                    asts = asts.large_text(config["tooltip"].as_str().unwrap());
                }
                if config["small_image"].as_str().is_some() {
                    asts = asts.large_image(config["small_image"].as_str().unwrap());
                }
                if config["small_tooltip"].as_str().is_some() {
                    asts = asts.large_text(config["small_tooltip"].as_str().unwrap());
                }
                asts
            })
        })
        {
            println!("Error: {}", e);
        }
        thread::sleep(interval);
    }
}

fn load_config(filename: &str) -> Yaml {
    let mut config_file = File::open(filename)
        .expect(&format!("Could not open file {}", filename));
    let mut s = String::new();
    config_file.read_to_string(&mut s)
        .expect(&format!("Could not read file {}", filename));
    return YamlLoader::load_from_str(&s)
        .expect(&format!("Invalid YAML in file {}", filename))[0].clone();
}
