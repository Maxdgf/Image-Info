/*
========================================================================
| This module contains functions for drawing and managing text screens.|
| {Functions:}                                                         |
| 1) clear_screen() - clears terminal using Term library               |
| 2) draw_main_screen() - draws main app screen                        |
| 3) draw_image_info_screen() - draws image info command result screen |
| 4) draw_help_screen() - draws screen with help panel                 |
| 5) draw_command_cursor() - draws command input cursor                |
| 6) draw_bye_message() - draws bye message cursor                     |
| 7) draw_help_screen() - draws help panel screen                      |
| 8) draw_image_exif_metadata_screen() - draws image exif metadata res |
| 9) draw_all_images_sizes_and_info() - draws all images with specific |
| extension sizes and info                                             |
| 10) draw_command_error_message() - draws input command error message |
| 11) reset_buffer() - resets buffer for print!()                      |
========================================================================
*/

use std::io::*;
use std::thread;
use std::time::Duration;
use std::process::Command;

use crate::modules::main_mechanics::image_manager::*;
use crate::modules::app_design_managment::decoration_patterns::*;
use crate::modules::ansi_styles_managment::ansi_escape_codes::*;
use crate::modules::ansi_styles_managment::ansi_styles_manager::*;
use crate::modules::app_design_managment::progress_bar::*;

//clears screen
fn clear_screen() {
    //checking os with cfg! macros during compilation
    if cfg!(target_os = "windows") {
        //for Windows
        Command::new("cmd")//cls command in windows built-in command cmd.exe, you can't call it as a separate program in Command::new().
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal before drawing app screen!");
    } else {
        //for UNIX sytems (Linux, macOS)
        Command::new("clear")
            .status()
            .expect("Failed to clear terminal before drawing app screen!");
    }
}

//draws main screen
pub fn draw_main_screen() {
    clear_screen();

    println!("{}", font_color(FONT_COLORS.f_cyan, APP_LOGO));
    println!("\n{}", app_description());
    println!("{}", app_author_panel());
    println!("{}", LINE);
    println!("{}", app_commands_list_panel());
    print!("{}", command_cursor("Enter command"));

    reset_buffer(); //resetting buffer for print!()
}

//draws image info command result screen 
pub fn draw_image_info_screen(
    path: String,
    task_name: &str
) {
    clear_screen();

    draw_progress_bar_title(
        "Get Image Info".to_string(),
    );

    //creating progress bar and starting
    let loading_bar = create_loading_spinner("Loading...".to_string());
    loading_bar.enable_steady_tick(Duration::from_millis(150));

    let get_image_data_handle = thread::spawn(move || {
        get_image_data(path)
    });
    let image_data = get_image_data_handle.join().unwrap();
    let mut input = String::new();

    loading_bar.finish(); //finishing

    clear_screen();

    println!("{}", task_view(task_name));
    println!("{}", image_data);
    print!("{}", command_cursor("Enter any key to continue"));

    reset_buffer(); //resetting buffer for print!()

    stdin().read_line(&mut input).expect("Failed to read input string!");

    //clears screen and draws main screen
    if !input.is_empty() {
        clear_screen();
        draw_main_screen();
    }
}

//draws help panel screen
pub fn draw_help_screen() {
    clear_screen();

    let mut input = String::new();

    println!("{}", help_panel());
    print!("{}", command_cursor("Enter any key to continue"));

    reset_buffer(); //reesetting buffer for print!()

    stdin().read_line(&mut input).expect("Failed to read input string!");

    //clears screen and draws main screen
    if !input.is_empty() {
        clear_screen();
        draw_main_screen();
    }
}

//draws bye message then exit command
pub fn draw_bye_message() {
    clear_screen();
    println!("{}", bye_message());
}

//draws input command error message then clears all for next drawing
pub fn draw_command_error_message(input_command: String) {
    println!("{}", unknown_command_message_cursor(input_command));

    thread::sleep(Duration::from_secs(1)); //little delay

    clear_screen(); //clear screen
}

//draws image exif metadata command result screen
pub fn draw_image_exif_metadata_screen(
    path: String,
    task_name: &str
) {
    clear_screen();

    draw_progress_bar_title("Get Image Exif Metadata".to_string());

    //creating progress bar and starting
    let loading_bar = create_loading_spinner("Loading...".to_string());
    loading_bar.enable_steady_tick(Duration::from_millis(150));

    let get_exif_metadata_handle = thread::spawn(move || {
        get_image_exif_metadata(path)
    });
    let exif_metadata = get_exif_metadata_handle.join().unwrap();
    let mut input = String::new();

    loading_bar.finish(); //finishing

    clear_screen();

    println!("{}", task_view(task_name));
    println!("{}", exif_metadata);
    print!("{}", command_cursor("Enter any key to continue"));

    reset_buffer(); //resetting buffer for print!()

    stdin().read_line(&mut input).expect("Failed to read input string!");

    //clears screen and draws main screen
    if !input.is_empty() {
        clear_screen();
        draw_main_screen();
    }
}

//draws all images sizes and info command result screen
pub fn draw_all_images_sizes_and_info(
    extension: String,
    task_name: &str
) {
    clear_screen();

    draw_progress_bar_title("Get All Images Sizes And Info".to_string());

    //creating progress bar and starting
    let loading_bar = create_loading_spinner("Loading...".to_string());
    loading_bar.enable_steady_tick(Duration::from_millis(150));

    let get_all_images_sizes_and_info_handle = thread::spawn(move || {
        get_images_size_with_extension(
            extension,
            loading_bar
        )
    });
    let info = get_all_images_sizes_and_info_handle.join().unwrap();
    let mut input = String::new();

    clear_screen();

    println!("{}", task_view(task_name));
    println!("{}", info);
    print!("{}", command_cursor("Enter any key to continue"));

    reset_buffer(); //resetting buffer for print!()

    stdin().read_line(&mut input).expect("Failed to read input string!");

    //clears screen and draws main screen
    if !input.is_empty() {
        clear_screen();
        draw_main_screen();
    }
}

//resets buffer for print!()
fn reset_buffer() {
    stdout().flush().unwrap();
}
