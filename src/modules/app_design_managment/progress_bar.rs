/*
=======================================================================
| This module contains functions for creating and manage progress bar.|
| Here uses libs: rand and indicatif.                                 |
| {Functions:}                                                        |
| 1) draw_progress_bar_title() - draws progress bar decorated title   |
| 2) create_loading_spinner() - creating indicatif progress spinner   |
=======================================================================
*/

use rand::Rng;
use indicatif::ProgressBar;

use crate::modules::ansi_styles_managment::ansi_escape_codes::{
    BACKGROUND_COLORS_LIST, FONT_COLORS_LIST, FONT_STYLES_LIST
};
use crate::modules::ansi_styles_managment::ansi_styles_manager::{
    background_color, font_color, font_style
};

//draws progress bar decorated title
pub fn draw_progress_bar_title(task_name: String) {
    let mut rnd = rand::rng();

    let fcs_index = rnd.random_range(0..FONT_COLORS_LIST.len());
    let bcl_index = rnd.random_range(0..BACKGROUND_COLORS_LIST.len());
    let fsl_index = rnd.random_range(0..FONT_STYLES_LIST.len());

    println!(
        "{}", 
        format!(
            "|[{}]-[{}]-({})",

            background_color(
                BACKGROUND_COLORS_LIST[bcl_index], 
                "     "
            ),

            font_color(
                FONT_COLORS_LIST[fcs_index], 
                "Please, wait."
            ),

            font_style(
                FONT_STYLES_LIST[fsl_index], 
                &task_name
            )
        )
    );  
}

//creating indicatif progress spinner (he is managed directly in the modules where he is needed (for greater and more convenient control and updates))
pub fn create_loading_spinner(msg: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(msg);

    return spinner
}