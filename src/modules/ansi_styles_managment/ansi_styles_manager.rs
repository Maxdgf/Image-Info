/*
=================================================================
| This module contains a functions for setting a different text |
| styles with ANSI escape codes.                                |
| {Functions:}                                                  |
| 1) font_color() - setts text color                            |
| 2) background_color() - setts text background color           |
| 3) font_style() - setts text style                            |
| 4) font_and_style() - setts text color and style              |
| 5) full_style() - setts text color, background and style      |
| 6) reset_all() - resets text color, background and style (all)|
=================================================================
*/

use super::ansi_escape_codes::RESET;

//parts of ansi escape code
//ESCAPE - start | m - end
const ESCAPE: &str = "\x1b[";
const END: char = 'm';

//creating ansi escape code for text font color
pub fn font_color(
    font_color_code: i32, 
    content: &str
) -> String {
    let string_code = font_color_code.to_string();

    let result = format!(
        "{}{}{}{}{}",
        ESCAPE,
        &string_code,
        END,
        content,
        &reset_all()
    );
    
    return result
}

 //creating ansi escape code for text background color
pub fn background_color(
    background_color_code: i32,
    content: &str
) -> String {
    let string_code = background_color_code.to_string();

    let result = format!(
        "{}{}{}{}{}",
        ESCAPE,
        &string_code,
        END,
        content,
        &reset_all()
    );
    
    return result
}

//creating ANSI escape code for text font style
pub fn font_style(
    style_code: i8,
    content: &str
) -> String {
    let string_code = style_code.to_string();

    let result = format!(
        "{}{}{}{}{}",
        ESCAPE,
        &string_code,
        END,
        content,
        &reset_all()
    );
    
    return result
}

//creating ANSI escape code for text font color and style
pub fn font_and_style(
    style_code: i8,
    font_code: i32,
    content: &str
) -> String {
    let font_string_code = font_code.to_string();
    let style_string_code = style_code.to_string();

    let result = format!(
        "{}{};{}{}{}{}",
        ESCAPE,
        &style_string_code,
        &font_string_code,
        END,
        content,
        &reset_all()
    );

    return result
}

//creating ANSI escape code for text font color, background and style
pub fn full_style(
    font_color_code: i32,
    background_color_code: i32,
    style_code: i8,
    content: &str
) -> String {
    let font_string_code = font_color_code.to_string();
    let background_string_code = background_color_code.to_string();
    let style_string_code = style_code.to_string();

    let result = format!(
        "{}{};{};{}{}{}{}",
        ESCAPE,
        &style_string_code,
        &font_string_code,
        &background_string_code,
        END,
        content,
        &reset_all()
    );

    return result
}

//resets all
fn reset_all() -> String {
    let result = format!(
        "{}{}{}",
        ESCAPE,
        RESET,
        END
    );

    return result
}