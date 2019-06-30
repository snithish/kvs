#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use kvs::KvStore;
use kvs::Result;
use std::env::current_dir;

fn main() -> Result<()> {
    const SET_COMMAND_NAME: &str = "set";
    const GET_COMMAND_NAME: &str = "get";
    const REMOVE_COMMAND_NAME: &str = "rm";

    let key_args = Arg::with_name("key").index(1).required(true);
    let value_args = Arg::with_name("value").index(2).required(true);
    let set_sub_command = SubCommand::with_name(SET_COMMAND_NAME)
        .arg(&key_args)
        .arg(&value_args)
        .about("Set a key to value mapping")
        .help("Add <key> and associated <value> to KVS");
    let get_sub_command = SubCommand::with_name(GET_COMMAND_NAME)
        .arg(&key_args)
        .about("Get value mapped to key")
        .help("Query to fetch value associated with <key> if present");
    let remove_sub_command = SubCommand::with_name(REMOVE_COMMAND_NAME)
        .arg(&key_args)
        .about("Remove associated value")
        .help("Remove <key> from KVS");

    let app: App = app_from_crate!();

    let matches = app
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommands(vec![set_sub_command, get_sub_command, remove_sub_command])
        .get_matches();

    let mut kvs_store = KvStore::open(current_dir()?)?;

    match matches.subcommand() {
        (SET_COMMAND_NAME, Some(set_matches)) => {
            let key = key_string(&set_matches);
            let value = set_matches.value_of("value").unwrap().to_string();
            kvs_store.set(key.clone(), value.clone())?;
            println!("set {} to {}", key, value);
            Ok(())
        }
        (GET_COMMAND_NAME, Some(get_command_matches)) => {
            let key = key_string(get_command_matches);
            let fetched_value = kvs_store.get(key.clone())?.unwrap();
            println!("{} is associated with key: {} ", fetched_value, key);
            Ok(())
        }
        (REMOVE_COMMAND_NAME, Some(remove_command_matches)) => {
            let key = key_string(&remove_command_matches);
            kvs_store.remove(key.clone())?;
            println!("{} removed from kvs", key);
            Ok(())
        }
        _ => unreachable!(),
    }
}

fn key_string(matches: &ArgMatches) -> String {
    let option = matches.value_of("key");
    option.map(|s| s.to_string()).unwrap()
}
