#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let key_args = Arg::with_name("key").index(1).required(true);
    let value_args = Arg::with_name("value").index(2).required(true);
    let set_sub_command = SubCommand::with_name("set")
        .arg(&key_args)
        .arg(&value_args)
        .help("Add <key> and associated <value> to KVS");
    let get_sub_command = SubCommand::with_name("get")
        .arg(&key_args)
        .help("Query to fetch value associated with <key> if present");
    let remove_sub_command = SubCommand::with_name("rm")
        .arg(&key_args)
        .help("Remove <key> from KVS");
    let matches = app_from_crate!()
        .subcommands(vec![set_sub_command, get_sub_command, remove_sub_command])
        .get_matches();
}
