/*
============================================================================
| This module contains decoration assets for application.                  |
| {Functions:}                                                             |
| 1) app_description() - returns app decription formatted text             |
| 2) task_view() - returns a task view for command result                  |
| 3) bye_message() - returns bye message for exit command                  |
| 4) command_cursor() - returns decorated command cursor for input field   |
| 5) unknown_command_message_cursor() - returns unknown message cursor when|
| unknown command                                                          |
| 6) app_author_panel() - returns app author decorated panel               |
| 7) help_panel() - returns help panel with info about commands            |
| 8) app_commands_list_panel() - returns decorated panel with commands     |
============================================================================
*/

use crate::modules::ansi_styles_managment::ansi_escape_codes::*;
use crate::modules::ansi_styles_managment::ansi_styles_manager::*;

pub const APP_LOGO: &str = r#"
 _____                             _____       __      
|_   _|                           |_   _|     / _|     
  | | _ __ ___   __ _  __ _  ___    | | _ __ | |_ ___  
  | || '_ ` _ \ / _` |/ _` |/ _ \   | || '_ \|  _/ _ \ 
 _| || | | | | | (_| | (_| |  __/  _| || | | | || (_) |
 \___/_| |_| |_|\__,_|\__, |\___|  \___/_| |_|_| \___/ 
                       __/ |                           
                      |___/         "#;
pub const LINE: &str = "| 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 |";

pub fn app_description() -> String {
    return font_color(
        FONT_COLORS.f_cyan, 
        "• Get information about image and more!"
    );    
}

pub fn task_view(task_name: &str) -> String {
    let task_view = format!(
        "|-[{}]-|",
        background_color(
            BACKGROUND_COLORS.b_green,
            task_name
        )
    );

    return task_view
}

pub fn bye_message() -> String {
    let bye_message = format!(
        "|-[{}]",
        background_color(
            BACKGROUND_COLORS.b_purple, 
            "Thanks for using!"
        )
    );

    return bye_message
}

pub fn command_cursor(description: &str) -> String {
    let cursor = format!("|[{}]-> ", font_style(
            FONT_STYLES.reverse, 
            description
        )
    );

    return cursor
}

pub fn unknown_command_message_cursor(incorrect_command: String) -> String {
    let message_cursor = format!(
        "|[{}]->[{}]->[{}]", 

        full_style(
            FONT_COLORS.f_white, 
            BACKGROUND_COLORS.b_red, 
            FONT_STYLES.bold, 
            "Unknown command!", 
        ),

        full_style(
            FONT_COLORS.f_white, 
            BACKGROUND_COLORS.b_gray, 
            FONT_STYLES.strike_through, 
            &incorrect_command, 
        ),

        full_style(
            FONT_COLORS.f_white, 
            BACKGROUND_COLORS.b_blue, 
            FONT_STYLES.bold, 
            "See commands list", 
        )
    );

    return message_cursor
}

pub fn app_author_panel() -> String {
    let author_name = "Maxdgf";
    let author_github_link = "https://github.com/Maxdgf";
    let app_author = format!(
"=========================================================
|            This tool created by: {}               |
|-------------------------------------------------------|
|           {} -> {}         |
=========================================================",
    font_color(
        FONT_COLORS.f_green, 
        author_name
    ), 

    full_style(
        FONT_COLORS.f_white, 
        BACKGROUND_COLORS.b_black, 
        FONT_STYLES.bold, 
        "Github"
    ),

    font_and_style(
        FONT_STYLES.italic, 
        FONT_COLORS.f_blue, 
        author_github_link
    ));  

    return app_author
}

pub fn help_panel() -> String {
    let help_panel = format!(
"=============================================================================================================================
|                                                           [{}]                                                          |
|---------------------------------------------------------------------------------------------------------------------------|
| {} | {}. Get info about image in form of dimensions, color model, filename and extension, file size.  |
| {} | {}. Fetch Exif metadata from image. Exif-supporting formats: [{}].   |
| {}  | {}. Get all images size with specific extension.                                                    |
| {}       | {}. Closing application.                                                                               |
=============================================================================================================================",

        font_and_style(
            FONT_STYLES.italic, 
            FONT_COLORS.f_light_purple, 
            "Help"
        ),
        
        font_and_style(
            FONT_STYLES.underline, 
            FONT_COLORS.f_light_white, 
            "gii=(path)"
        ),

        font_and_style(
            FONT_STYLES.bold, 
            FONT_COLORS.f_light_white, 
            "Get Image Info", 
        ),

        font_and_style(
            FONT_STYLES.underline, 
            FONT_COLORS.f_light_white, 
            "fem=(path)", 
        ),

        font_and_style(
            FONT_STYLES.bold, 
            FONT_COLORS.f_light_white, 
            "Fetch Exif Metadata"
        ),

        font_and_style(
            FONT_STYLES.italic, 
            FONT_COLORS.f_light_yellow, 
            ".JPEG, .TIFF, .PNG, .WEBP"
        ),

        font_and_style(
            FONT_STYLES.underline, 
            FONT_COLORS.f_light_white, 
            "is=(type)", 
        ),

        font_and_style(
            FONT_STYLES.bold, 
            FONT_COLORS.f_light_white, 
            "Images Size"
        ),

        font_and_style(
            FONT_STYLES.underline, 
            FONT_COLORS.f_light_purple, 
            "exit"
        ),

        font_and_style(
            FONT_STYLES.bold, 
            FONT_COLORS.f_light_red, 
            "Exit app"
        )
    );

    return  help_panel
}

pub fn app_commands_list_panel() -> String {
    let commands_list_panel = format!(
"====================================================================
| {} | {} | {}               |
| {} | {} | {}           |
| {} | {} | {}  |
| {} | {} | {}                                    |
| {} | {} | {}                                     |
====================================================================", 
    font_color(
        FONT_COLORS.f_yellow, 
        "[1]"
    ),
    font_style(
        FONT_STYLES.reverse, 
        "gii=(path)"
    ),
    font_and_style(
        FONT_STYLES.underline, 
        FONT_COLORS.f_light_green, 
        "Get info about image from path."
    ),

    font_color(
        FONT_COLORS.f_yellow, 
        "[2]"
    ),
    font_style(
        FONT_STYLES.reverse, 
        "fem=(path)"
    ),
    font_and_style(
        FONT_STYLES.underline, 
        FONT_COLORS.f_light_green, 
        "Fetch Exif image matadata from path"
    ),

    font_color(
        FONT_COLORS.f_yellow, 
        "[3]"
    ),
    font_style(
        FONT_STYLES.reverse, 
        "is=(type) "
    ),
    font_and_style(
        FONT_STYLES.underline, 
        FONT_COLORS.f_light_green, 
        "Get all images size with specific extension."
    ),

    font_color(
        FONT_COLORS.f_yellow, 
        "[4]"
    ),
    font_style(
        FONT_STYLES.reverse, 
        "help      "
    ),
    font_and_style(
        FONT_STYLES.underline, 
        FONT_COLORS.f_light_purple, 
        "Show help."
    ),

    font_color(
        FONT_COLORS.f_yellow, 
        "[5]"
    ),
    font_style(
        FONT_STYLES.reverse, 
        "exit      "
    ),
    font_and_style(
        FONT_STYLES.underline, 
        FONT_COLORS.f_light_red, 
        "Exit app."
    ));

    return commands_list_panel
}