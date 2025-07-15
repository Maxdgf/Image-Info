/*
==============================================================
| This module contains ANSI escape codes for text decorating |
==============================================================
*/

//reset all styles code
pub const RESET: u8 = 0;

//ansi text font color codes struct
pub struct AnsiFontColors {
    pub f_red: i32,
    pub f_green: i32,
    pub f_blue: i32,
    pub f_black: i32,
    pub f_yellow: i32,
    pub f_cyan: i32,
    pub f_purple: i32,
    pub f_white: i32,
    pub f_gray: i32,
    pub f_light_red: i32,
    pub f_light_green: i32,
    pub f_light_blue: i32,
    pub f_light_yellow: i32,
    pub f_light_purple: i32,
    pub f_light_white: i32
}

//ansi text background color codes struct
pub struct AnsiBackgroundColors {
    pub b_red: i32,
    pub b_green: i32,
    pub b_blue: i32,
    pub b_black: i32,
    pub b_yellow: i32,
    pub b_cyan: i32,
    pub b_purple: i32,
    pub b_white: i32,
    pub b_gray: i32,
    pub b_light_red: i32,
    pub b_light_green: i32,
    pub b_light_blue: i32,
    pub b_light_yellow: i32,
    pub b_light_purple: i32,
    pub b_light_white: i32
}

//ansi text font styles struct
pub struct AnsiFontStyles {
    pub bold: i8,
    pub italic: i8,
    pub underline: i8,
    pub reverse: i8,
    pub dim: i8,
    pub strike_through: i8
}

//initializing an ansi text font colors struct
pub const FONT_COLORS: AnsiFontColors = AnsiFontColors {
    f_red: 31,
    f_green: 32,
    f_blue: 34,
    f_black: 30,
    f_yellow: 33,
    f_cyan: 36,
    f_purple: 35,
    f_white: 37,
    f_gray: 90,
    f_light_red: 91,
    f_light_green: 92,
    f_light_blue: 94,
    f_light_yellow: 93,
    f_light_purple: 95,
    f_light_white: 97
};

//initializing an ansi text background colors struct
pub const BACKGROUND_COLORS: AnsiBackgroundColors = AnsiBackgroundColors {
    b_red: 41,
    b_green: 42,
    b_blue: 44,
    b_black: 40,
    b_yellow: 43,
    b_cyan: 46,
    b_purple: 45,
    b_white: 47,
    b_gray: 100,
    b_light_red: 101,
    b_light_green: 102,
    b_light_blue: 104,
    b_light_yellow: 103,
    b_light_purple: 105,
    b_light_white: 107
};

//initializing an ansi text styles struct
pub const FONT_STYLES: AnsiFontStyles = AnsiFontStyles {
    bold: 1,
    italic: 3,
    underline: 4,
    reverse: 7,
    dim: 2,
    strike_through: 9
};

//ansi font colors array
pub const FONT_COLORS_LIST: [i32; 16] = [
    FONT_COLORS.f_red, FONT_COLORS.f_green,
    FONT_COLORS.f_blue, FONT_COLORS.f_black,
    FONT_COLORS.f_yellow, FONT_COLORS.f_cyan,
    FONT_COLORS.f_purple, FONT_COLORS.f_white,
    FONT_COLORS.f_gray, FONT_COLORS.f_light_red,
    FONT_COLORS.f_light_green, FONT_COLORS.f_light_blue,
    FONT_COLORS.f_light_yellow, FONT_COLORS.f_purple,
    FONT_COLORS.f_light_white, FONT_COLORS.f_light_purple
];

//ansi background colors array
pub const BACKGROUND_COLORS_LIST: [i32; 16] = [
    BACKGROUND_COLORS.b_red, BACKGROUND_COLORS.b_green,
    BACKGROUND_COLORS.b_blue, BACKGROUND_COLORS.b_black,
    BACKGROUND_COLORS.b_yellow, BACKGROUND_COLORS.b_cyan,
    BACKGROUND_COLORS.b_purple, BACKGROUND_COLORS.b_white,
    BACKGROUND_COLORS.b_gray, BACKGROUND_COLORS.b_light_red,
    BACKGROUND_COLORS.b_light_green, BACKGROUND_COLORS.b_light_blue,
    BACKGROUND_COLORS.b_light_yellow, BACKGROUND_COLORS.b_purple,
    BACKGROUND_COLORS.b_light_white, BACKGROUND_COLORS.b_light_purple
];

//ansi font styles array
pub const FONT_STYLES_LIST: [i8; 6] = [
    FONT_STYLES.bold, FONT_STYLES.italic,
    FONT_STYLES.reverse, FONT_STYLES.underline,
    FONT_STYLES.dim, FONT_STYLES.strike_through
];