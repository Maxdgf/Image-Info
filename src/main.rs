/*
================================================================
|                        Image Info                            |
|--------------------------------------------------------------|
| CLI application for get different info about image or images.|
|--------------------------------------------------------------|
| by Maxdgf, GitHub -> https://github.com/Maxdgf               |
| 14.07.2025                                                   |
================================================================ 
*/

mod modules;

use std::io::stdin;

use modules::app_design_managment::screens_manager::*;
use modules::main_mechanics::command_parser::parse_command;

//gii=(C:\Users\maxon\Downloads\photo_2025-07-09_22-14-41.jpeg

fn main() {
    draw_main_screen(); //draws main screen

    //main cycle
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input string!"); //input command line

        parse_command(&input); //parses input command
    }       
}