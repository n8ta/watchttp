extern crate yaml_rust;

use std::env;
use std::process::exit;
use watchttp::config::{Config, parse_config_file};
use watchttp::notif::{send_notification};
use std::thread::sleep;
use std::collections::HashMap;
use watchttp::http::download_site;
use watchttp::{SITES_KEY, TIMING_KEY};
use std::cmp::Ordering;
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use std::sync::mpsc::{channel};


fn print_help() {
    eprintln!("> ./watchttp /path/to/config.yaml\n");
    eprintln!("Config file format:");
    eprintln!("# config.yaml");
    eprintln!("{}: ", SITES_KEY);
    eprintln!("    - https://google.com");
    eprintln!("    - https://n8ta.com");
    eprintln!("{}: 36000000", TIMING_KEY);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &String = match args.get(1) {
        None => {
            eprintln!("Not enough args, provide a path to a config file.");
            print_help();
            exit(-1);
        }
        Some(path) => path
    };

    let mut config: Config = match parse_config_file(path) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("{}", msg);
            exit(-1);
        }
    };
    let mut stored_versions: HashMap<String, String> = Default::default();

    let (sender, receiver) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        match res {
            Ok(_) => {
                match sender.send(true) {
                    Err(_) => {
                        eprintln!("Failed to notify main thread of config file update");
                        send_notification("Failed to reload config file", "Sending to main thread failed.");
                    }
                    _ => {}
                }
            }
            Err(_) => {
                eprintln!("Something went wrong watching the config file.");
                send_notification("Something went wrong watching the config file.",
                                  "Something went wrong watching the config file.");
            }
        }
    }).expect("Failed to create new file watcher for config file.");
    match watcher.watch(path, RecursiveMode::NonRecursive) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to create watcher for config file");
            exit(-1);
        }
    }


    loop {
        if let Ok(_) = receiver.try_recv() {
            match parse_config_file(path) {
                Ok(cnf) => {
                    config = cnf;
                    send_notification("Reloaded config file", "Success!");
                }
                Err(msg) => {
                    eprintln!("{}", msg);
                }
            }
            println!("Reloading config file: {}", path);
            send_notification("Reloading config", &path);
        }

        for site in config.sites.iter() {
            println!("Loading site {}", site);
            let new_body = match download_site(&site[..]) {
                Ok(contents) => contents,
                Err(e) => {
                    eprintln!("Error: Site '{}' wasn't able to be loaded. \n\tMessage: `{}`", site, e);
                    continue;
                }
            };
            if let Some(old_body) = stored_versions.get(site) {
                if old_body != &new_body {
                    let difference = (new_body.len() as i64) - (old_body.len() as i64);
                    let byte_or_bytes = match difference == 1 {
                        true => "byte",
                        false => "bytes"
                    };
                    let message = match old_body.len().cmp(&new_body.len()) {
                        Ordering::Less => format!("Got {} {} larger", new_body.len() - old_body.len(), byte_or_bytes),
                        Ordering::Equal => format!("Is the same size but content changed"),
                        Ordering::Greater => format!("Got {} {} smaller", old_body.len() - new_body.len(), byte_or_bytes)
                    };
                    send_notification(&format!("{} changed", site),
                                      &message[..]);
                }
            }
            stored_versions.insert(site.clone(), new_body);
        }
        sleep(config.period);
    }
}
