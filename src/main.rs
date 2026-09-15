#![allow(unused_macros)]
#![allow(unused_imports)]
#[macro_use]
mod macros;

use lazy_static::lazy_static;
use std::{env::args, ffi::os_str::Display, fs::read_to_string, io::Lines, sync::{Arc, RwLock}};
use tinterm::{Color, Gradient};
use colored_text::Colorize;
use clearscreen::{self, clear};
use std::{io,io::Write, process::exit};
use std::process::Command;
use std::fs::read;

lazy_static! {
    static ref CONFIG_PATH: RwLock<String> = RwLock::new(String::from("config.json"));
}


fn main() {
    loop {
        clear().expect("yo phone is linging");
        print_cyan!("Choix: \n  1 - Ajouter un alias a la config fish\n  2 - Lire la config fish actuele (Lecture seule par défaut!)\n  3 - Listage des alias dans la config");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read input");

        if action.trim() == "1" {
            print_info!("Entrez la ligne a ajouter a la config (Comme un alias!)\n");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("You failed");

            let mut f = std::fs::OpenOptions::new()
                .append(true) // writes always go to the end of the file
                .open("/home/blahaj/Projects/Bash/auto-alias/Bash tests/config.fish")
                .expect("Error writing");

            writeln!(f, "{input}").expect("Error writing 2");
            let input2 = input.trim();

            print_success!("Alias: [ {input2} ] ajouté! Pressez la touche entrer pour retourner au menu!");

            let mut sleep = String::new();
            io::stdin()
                .read_line(&mut sleep)
                .expect("skill issues");
        }

        let parts: Vec<&str> = action.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["2"] => {
                let output = Command::new("nano")
                    .arg("-v")
                    .arg("Bash tests/config.fish") // the file to open
                    .status() // .status() waits; .spawn() backgrounds
                    .expect("failed to run nano");

                if !output.success() {
                    eprintln!("nano exited with an error");
                }
            }
            ["2", "edit"] => {
                let output = Command::new("nano")
                    .arg("Bash tests/config.fish") // the file to open
                    .status() // .status() waits; .spawn() backgrounds
                    .expect("failed to run nano");

                if !output.success() {
                    eprintln!("nano exited with an error");
                }
            }
            _ => {}
        }

        if action.trim() == "3" {
            let f = "Bash tests/config.fish";
            let pattern = "alias";
            let test = read_to_string(f).expect("Error reading 2").to_lowercase();
            
            let matching_lines = test
                .lines()
                .filter(|line|line.to_lowercase().contains(&pattern.to_lowercase()));
            
            
            //print_cyan!("test value: {test}");
            for line in matching_lines {
                print_cyan!("{line}");
            }
            let mut sleep = String::new();
            io::stdin()
                .read_line(&mut sleep)
                .expect("skill issues");
            } 
        

        else {
        }
    }
}