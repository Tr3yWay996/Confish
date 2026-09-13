#![allow(unused_macros)]
#![allow(unused_imports)]

use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};
use tinterm::{Color, Gradient};
use colored_text::Colorize;
use clearscreen::{self, clear};
use std::{io, process::exit};

lazy_static! {
    static ref CONFIG_PATH: RwLock<String> = RwLock::new(String::from("config.json"));
}
// Color preset for error messages, warnings, informations, and sucess / confirmation messages
macro_rules! print_error {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).red())
    };
}
macro_rules! print_info {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).blue())
    };
}
macro_rules! print_warn {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).yellow())
    };
}
macro_rules! print_success {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).green())
    };
}
// Color macro for primary, secondary, tertiary message type
macro_rules! print_primary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 194, 159))
    };
}
macro_rules! print_secondary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 172, 125))
    };
}
macro_rules! print_tertiary {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(255, 149, 90))
    };
}
// Color macro do just about any RGB or HEX color possible ever to be used in the final executable
macro_rules! print_cyan {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).cyan())
    };
}
macro_rules! print_purple {
    ($($arg:tt)*) => {
        println!("{}", format!($($arg)*).rgb(208, 0, 255))
    };
}

fn main() {
    clear().expect("yo phone is linging");
    print_primary!("Choix: \n1 - Ajouter un alias a la config fish\n 2- Lire la config fish actuele");
    let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");

        if action.trim() == "1" {
            print_error!("ERROR!")
        }
}