/*
================================================================
| This module parse commands and launchs current functions.    |
| {Functions:}                                                 |
| 1) parse_command() - parses commands from user, uses regexes |
| patterns (Regex library)                                     |
================================================================
*/

use std::thread;
use std::process;
use std::time::Duration;

use regex::Regex;

use crate::modules::app_design_managment::screens_manager::*;

pub fn parse_command(input: &str) {
    let input_command = input.trim(); //trim input

    //commands (used regexes for better commands syntax and functionality)
    let gii_command_regex = Regex::new(r"^gii=\(([^()]+)\)$").unwrap();
    let fem_command_regex = Regex::new(r"^fem=\(([^()]+)\)$").unwrap();
    let is_command_regex = Regex::new(r"^is=\(([^()]+)\)$").unwrap();

    //processing commands
    if let Some(text) = gii_command_regex.captures(input_command) {
        let path = &text[1]; //getting value from () command

        //Launch function
        draw_image_info_screen(
            path.to_string(),
            "Get Image Info"
        );
    } else if let Some(text) = fem_command_regex.captures(input_command) {
        let path = &text[1]; //getting value from () command
        
        //Launch function
        draw_image_exif_metadata_screen(
            path.to_string(), 
            "Get image Exif metadata"
        );
    } else if let Some(text) = is_command_regex.captures(input_command) {
        let path = &text[1]; //getting value from () command
           
        //Launch function
        draw_all_images_sizes_and_info( 
            path.to_string(), 
            "test"
        );
    } else {
        //processing other commands (without regexes)
        if input_command == "help" {
            draw_help_screen();
        } else if input_command == "exit" {
            draw_bye_message();

            thread::sleep(Duration::from_secs(2)); //little delay before turn off application

            process::exit(0); //turn off application
        } else {
            //processing empty input
            //note: if app not recognize command, printing "unknown command message"->(|[Unknown command!]->[unknown command]->[See commands list]) here "[unknown command]" unknown command from user and if command from user is empty => printing "~empty~""
            if !input_command.is_empty() {
                draw_command_error_message(input_command.to_string()); //if command not empty, focus on command
                draw_main_screen();
            } else {
                draw_command_error_message("~empty~".to_string()); //if command is empty, printing "~empty~"
                draw_main_screen();
            }    
        }
    }
}