/*
========================================================================================
| This module parse functions for processing image, fetching metadata                  |
| and get info about images sizes, count and all size with specific extension from dirs|
| used libs: [image-image processing, rexif-fetching exif metadata, rand,              |
| [dirs-crossplatform methods to get dirs addresses, walkdir-for directory crawling]   |
| {Functions:}                                                                         |
| 1) check_image_format() - checks image format for processing                         |
| 2) adapt_file_size() - converts file size in bytes to MB, KB, GB                     |      
| 3) convert_to_percent() - converts value to percent                                  |
| 4) generate_filename() - generates uniqie filename                                   |
| 5) get_all_image_pixels_info() - scans pixels. countы pixels, counting rgb and       |
| other colors percent                                                                 |
| 6) load_image() - loads image                                                        |
| 7) get_image_data() - gets image simple data                                         |
| 8) get_image_exif_metadata() - extracts Exif metadata from image                     |
| 9) get_images_size_with_extension() - scans computer dirs, counts image files and    |
| size with specific extension. counts all image files size                            |                
========================================================================================
*/

use dirs;
use indicatif::ProgressBar;
use rexif;
use image::*;
use rand::Rng;
use walkdir::WalkDir;

use std::fs::File;
use std::io::Write;
use std::borrow::Cow;
use std::path::{ Path, PathBuf };

use crate::modules::ansi_styles_managment::ansi_escape_codes::*;
use crate::modules::ansi_styles_managment::ansi_styles_manager::*;

const SUPPORTED_EXIF_METADATA_FORMATS: &[&str; 7] = &[
    "jpeg", "jpg", "tif", "tiff", "webp",
    "heic", "heif"
];

const IMAGE_FILES_FORMATS: &[&str; 16] = &[
    "png", "jpg", "jpeg", "gif", "webp",
    "raw", "tiff", "tif", "svg", "heic",
    "heif", "ico", "gif", "bmp", "psd",
    "avif"
];

//checks image format support
fn check_image_format_support(
    extension: &str,
    support_formats: &[&str]
) -> bool {
    let mut is_format_support = false;

    for format in support_formats {
        if &extension == format {
            is_format_support = true;
        }
    }

    return is_format_support
}

//adapts file size to readable size
fn adapt_file_size(file_size_in_bytes: f64) -> String {
    let mut result = String::new();

    const BYTES_IN_GIGABYTE: f64 = 1073741824.0;
    const BYTES_IN_MEGABYTE: f64 = 1048576.0;
    const BYTES_IN_KILOBYTE: f64 = 1024.0;

    if file_size_in_bytes >= BYTES_IN_GIGABYTE {
        //returns file size in gigabytes
        result.push_str(
            &format!(
                "{:.2} gb",
                file_size_in_bytes / BYTES_IN_GIGABYTE
            )
        );
    } else if file_size_in_bytes >= BYTES_IN_MEGABYTE {
        //returns file size in megabytes
        result.push_str(
            &format!(
                "{:.2} mb",
                file_size_in_bytes / BYTES_IN_MEGABYTE
            )
        );
    } else if file_size_in_bytes >= BYTES_IN_KILOBYTE {
        //returns file size in kilobytes
        result.push_str(
            &format!(
                "{:.2} kb",
                file_size_in_bytes / BYTES_IN_KILOBYTE
            )
        );
    } else {
        //returns file size in bytes
        result.push_str(
            &format!(
                "{} b",
                file_size_in_bytes
            )
        );
    }

    return result
}

//converts value to percent
fn convert_to_percent(
    max: i32,
    value: i32
) -> String {
    return (value / max * 100).to_string()
}

//generates uniqie filename
fn generate_filename(
    theme: &str,
    filename: &str,
    extension: &str
) -> String {
    let mut rnd = rand::rng();

    //random num for filename
    let num_index = rnd.random_range(0..100000);
    let result = format!(
        "{}  {}  {}{}",
        theme,
        filename,
        num_index,
        extension
    );

    return result
}

//processing image pixels data
fn get_all_image_pixels_info(
    width: u32,
    height: u32,
    image: DynamicImage
) -> String {
    struct RGBImageColors {
        red: i32,
        green: i32,
        blue: i32,
        other: i32
    }

    let mut pixels_count = 0;
    let mut result = String::new();
    let mut rgb_image_pixels_count = RGBImageColors {
        red: 0,
        green: 0,
        blue: 0,
        other: 0
    };

    let rgb_image = image.to_rgba8(); //converting immage to RGB, for pixels counting

    //scanning image for counting and get info about image pixels
    for y in 0..height {
        for x in 0..width {
            pixels_count += 1;

            let pixel = rgb_image.get_pixel(x, y);
            let channels = pixel.channels();

            //counting rgb colors colors count in pixel
            match channels {
                [255, 0, 0] => rgb_image_pixels_count.red += 1,
                [0, 255, 0] => rgb_image_pixels_count.blue += 1,
                [0, 0, 255] => rgb_image_pixels_count.green += 1,
                _ => rgb_image_pixels_count.other += 1
            }
        }
    }

    result.push_str(
        &format!(
            "|-[{}]-> ({} %)\n|-[{}]-> ({} %)\n|-[{}]-> ({} %)\n|-[{}]-> ({} %)",

            font_and_style(
                FONT_STYLES.bold, 
                FONT_COLORS.f_red, 
                "Red"
            ),
            font_style(
                FONT_STYLES.bold, 
                &convert_to_percent(pixels_count, rgb_image_pixels_count.red)
            ),

            font_and_style(
                FONT_STYLES.bold, 
                FONT_COLORS.f_green, 
                "Green"
            ),
            font_style(
                FONT_STYLES.bold, 
                &convert_to_percent(pixels_count, rgb_image_pixels_count.green)
            ),

            font_and_style(
                FONT_STYLES.bold, 
                FONT_COLORS.f_blue, 
                "Blue"
            ),
            font_style(
                FONT_STYLES.bold, 
                &convert_to_percent(pixels_count, rgb_image_pixels_count.blue)
            ),

            font_and_style(
                FONT_STYLES.bold, 
                FONT_COLORS.f_gray, 
                "Other"
            ),
            font_style(
                FONT_STYLES.bold, 
                &convert_to_percent(pixels_count, rgb_image_pixels_count.other)
            ),
        )
    );

    return result
}

//loads image from path
fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    let image = image::open(&Path::new(path))?;
    Ok(image)
}

//gets image data
pub fn get_image_data(path: String) -> String {
    let mut result = String::new();

    let trimmed_path = path.trim(); //trim path
    let image_path = Path::new(trimmed_path); //creating image path 

    let file_name = if let Some(name) = image_path.file_name() {
        name.to_string_lossy()
    } else {
        Cow::Borrowed("Unknown filename")
    }; //getting image filename

    let file_extension = if let Some(extension) = image_path.extension() {
        extension.to_string_lossy()
    } else {
        Cow::Borrowed("Unknown file extension")
    }; //getting image file extension

    //checking if it is image before processing
    let is_this_image_file = check_image_format_support(&file_extension, IMAGE_FILES_FORMATS);

    //if it is file image, go.
    if is_this_image_file {
        //load image from path and proccesing image (if result is Ok -> processing else return error message)
        match load_image(trimmed_path) {
            Ok(img) => {
                let (image_width, image_height) = img.dimensions(); //gets width and height
                let image_format = img.color(); //gets color format

                let file_size = image_path.metadata().unwrap().len() as f64 
                / (1024.0 * 1024.0); //getting image file size in mb
                let edited_file_size = adapt_file_size(file_size); //adapted file size, see adapt_file_size()

                let all_pixels_info = get_all_image_pixels_info(
                    image_width, 
                    image_height,
                    img
                ); //gettin image pixels data

                //push result to result string
                result.push_str(
                    &format!(
                        "|-Image dimensions: ({}x{})px\n|-Image color model: {:?}\n|-Image file name: {}\n|-Image file extension: {}\n|-Image file size: {}\n|-Image pixels info:\n{}\n|", 
                        image_width,
                        image_height,
                        image_format,
                        file_name,
                        file_extension,
                        edited_file_size,
                        all_pixels_info
                    )
                );
            }

            Err(error) => {
                //image couldn't open, push error message to result string
                let error_message = format!(
                    "|-[{}] Image from path [{}] couldn't open.\n|-{}\n|", 

                    full_style(
                        FONT_COLORS.f_white, 
                        BACKGROUND_COLORS.b_red, 
                        FONT_STYLES.bold, 
                        "Error!"
                    ),
                    
                    background_color(
                        BACKGROUND_COLORS.b_light_green, 
                        &path
                    ),

                    error
                );

                result.push_str(&error_message);
            }
        }
    } else {
        //if this file not image, push error message to result string
        result.push_str(
            &format!(
                "|[{}]->({})->This is not image!",

                full_style(
                    FONT_COLORS.f_white, 
                    BACKGROUND_COLORS.b_red, 
                    FONT_STYLES.bold, 
                    "Error!"
                ),
                font_style(
                    FONT_STYLES.underline, 
                    &file_extension
                )
            )
        );
    }

    return result
}

//gets image exif metadata
pub fn get_image_exif_metadata(path: String) -> String {
    let mut result = String::new();

    let trimmed_path = path.trim(); //trim path
    let image_path = Path::new(trimmed_path); //creating image path

    let file_name = if let Some(name) = image_path.file_name() {
        name.to_string_lossy()
    } else {
        Cow::Borrowed("Unknown name")
    }; //getting filename

    let file_extension = if let Some(extension) = image_path.extension() {
        extension.to_string_lossy()
    } else {
        Cow::Borrowed("Unknown file extension")
    }; //getting file extension

    //checking if it is file supporting exif metadata
    let is_format_exif_supported = check_image_format_support(&file_extension, SUPPORTED_EXIF_METADATA_FORMATS);

    if is_format_exif_supported {
        //fetching exif metadata

        //configuring output text file
        let mut output_file_directory = if let Some(output_directory) = dirs::download_dir() {
            output_directory.clone()
        } else {
            PathBuf::from("Unknown directory")
        }; //file save directory (download directory)

        output_file_directory.push(
            generate_filename(
                "Exif_meta", 
                &file_name, 
                ".txt"
            )
        ); //filename (txt file)

        let str_output_path = output_file_directory.clone().into_os_string().into_string().unwrap(); //output result file path converted to String

        //creating output result txt file
        let mut output_file = File::create(output_file_directory).expect("Couldn't create image Exif metadata output file");
        
        //parse Exif metadata from image
        let parsed_exif = rexif::parse_file(&*path);
        
        //if exif parsed succesfully, go.
        match parsed_exif {
            //if exif parsed, configuring result
            Ok(exif) => {
                let mut index = 0;
                let mut exif_data_to_save = String::new();

                let exif_entries = exif.entries; //getting exif entries
                let exif_entries_count = exif_entries.len(); //getting exif entries count

                //configuring result
                //creating description title
                result.push_str(
                    &format!(
                        "|-Image {} metadata fetched succesfully!\n|-Image filename-> {}, type-> {}, exif entries-> {}\n|===================================================|\n|\n",

                        font_style(
                            FONT_STYLES.bold, 
                            "Exif"
                        ),

                        &file_name,
                        &file_extension,
                        exif_entries_count
                    )
                );

                //fetch entries
                //configuring entry info string
                for entry in exif_entries {
                    index += 1;

                    let configuration = format!(
                        "|-[{}]->|{}|\t{}\n",
                        index,
                        entry.tag,
                        entry.value_more_readable
                    );

                    exif_data_to_save.push_str(&configuration);
                    result.push_str(&configuration);

                    if index == exif_entries_count {
                        result.push('|');
                    }
                }

                //writing entries to output txt file
                output_file.write(exif_data_to_save.as_bytes()).expect("Failed to write Exif metadata from image in result file.");

                //push output file description to result string
                result.push_str(
                    &format!(
                        "\n|-[{}]-> See in path: {}\n|",

                        full_style(
                            FONT_COLORS.f_white, 
                            BACKGROUND_COLORS.b_light_yellow, 
                            FONT_STYLES.bold, 
                            "Output txt file created!"
                        ),

                        full_style(
                            FONT_COLORS.f_light_white, 
                            BACKGROUND_COLORS.b_black, 
                            FONT_STYLES.italic,    
                            &str_output_path
                        )
                    )
                );
            }

            Err(error) => {
                //push error, Exif metadata not fetched message to result string
                result.push_str(
                    &format!(
                        "|-Image {} Exif metadata not fetched!\n|-[{}]->{}\n|",

                        font_style(
                            FONT_STYLES.bold,
                            "Exif"
                        ),

                        full_style(
                            FONT_COLORS.f_white, 
                            BACKGROUND_COLORS.b_red, 
                            FONT_STYLES.bold, 
                            "Error!"
                        ),

                        &error
                    )
                );
            }
        }
    } else {
        //printing exif-supporting formats error message
        result.push_str(
            &format!(
                "|-Sorry, this image format-> |{}| not support {} metadata.\n|-Supporting formats -> [{}]\n|",
                &file_extension,

                font_style(
                    FONT_STYLES.bold, 
                    "Exif"
                ),

                full_style(
                    FONT_COLORS.f_black, 
                    BACKGROUND_COLORS.b_light_blue, 
                    FONT_STYLES.bold, 
                    "JPEG, JPG, TIFF, HEIF, WEBP"
                )
            )
        );
    }

    return result
}

//gets images size with specific extension
pub fn get_images_size_with_extension(
    extension: String,
    progress_bar: ProgressBar
) -> String {
    //all dirs for scan
    let all_dirs: [Option<PathBuf>; 7] = [
        dirs::download_dir(),
        dirs::document_dir(),
        dirs::video_dir(),
        dirs::picture_dir(),
        dirs::data_local_dir(),
        dirs::data_dir(),
        dirs::desktop_dir()
    ];

    struct ImageFilesFromDirsCounts {
        download: i32,
        document: i32,
        video: i32,
        picture: i32,
        data_local: i32,
        data: i32,
        desktop: i32
    }

    struct ImageFilesSizesDirs {
        download: f64,
        document: f64,
        video: f64,
        picture: f64,
        data_local: f64,
        data: f64,
        desktop: f64
    }

    let mut image_files_dirs_counts = ImageFilesFromDirsCounts { 
        download: 0, 
        document: 0, 
        video: 0, 
        picture: 0, 
        data_local: 0, 
        data: 0,
        desktop: 0
    };

    let mut image_files_sizes_dirs = ImageFilesSizesDirs {
        download: 0.0,
        document: 0.0,
        video: 0.0,
        picture: 0.0,
        data_local: 0.0,
        data: 0.0,
        desktop: 0.0
    };
    
    let mut scanned_dirs = 0;
    let mut images_count = 0;
    let mut result = String::new();

    //checking if it is image
    let is_image_format_support = check_image_format_support(&extension, IMAGE_FILES_FORMATS);

    if is_image_format_support {
        //scanning dirs on images with specific extension
        for dir_index in 0..all_dirs.len() - 1 {
            scanned_dirs += 1; //scanned dirs count

            let current_dir = if let Some(directory) = &all_dirs[dir_index] {
                directory.clone()
            } else {
                PathBuf::from("Unknown directory")
            };

            let progress_bar_message = format!(
                "Scanned dirs ({})/({}) => images found: [{}]\ndir: • {:?} 🔎", 
                scanned_dirs, 
                all_dirs.len(),
                images_count,
                current_dir
            );

            progress_bar.set_message(progress_bar_message);

            //scanning files in current dir
            for entry in WalkDir::new(
                current_dir.into_os_string().into_string().unwrap()
            ).into_iter().filter_map(|e| e.ok()) {
                //checking if it is file for optimize scan work, if it is not needed, remove this check
                if entry.file_type().is_file() {
                    if let Some(ftype) = entry.path().extension() {
                        //if file extension equals current extension, count.
                        if ftype == &*extension {
                            images_count += 1;

                            let image_file_size = entry.path().metadata().unwrap().len() as f64;

                            //counting files in dir count and files size in dir
                            match dir_index {
                                0 => {
                                    image_files_dirs_counts.download += 1;
                                    image_files_sizes_dirs.download += image_file_size;
                                },
                                1 => {
                                    image_files_dirs_counts.document += 1;
                                    image_files_sizes_dirs.document += image_file_size;
                                },
                                2 => {
                                    image_files_dirs_counts.video += 1;
                                    image_files_sizes_dirs.video += image_file_size;
                                },
                                3 => {
                                    image_files_dirs_counts.picture += 1;
                                    image_files_sizes_dirs.picture += image_file_size;
                                },
                                4 => {
                                    image_files_dirs_counts.data_local += 1;
                                    image_files_sizes_dirs.data_local += image_file_size;
                                },
                                5 => {
                                    image_files_dirs_counts.data += 1;
                                    image_files_sizes_dirs.data += image_file_size;
                                },
                                6 => {
                                    image_files_dirs_counts.desktop += 1;
                                    image_files_sizes_dirs.desktop += image_file_size;
                                },
                                _ => images_count += 0 //default operation
                            }
                        }    
                    }    
                }    
            }
        }

        //all files size
        let all_size = 
            image_files_sizes_dirs.download + image_files_sizes_dirs.document +
            image_files_sizes_dirs.video + image_files_sizes_dirs.picture +
            image_files_sizes_dirs.data_local + image_files_sizes_dirs.data +
            image_files_sizes_dirs.desktop;

        if images_count > 0 {
            result.push_str(
                &format!(
                    "|Found ({}) image files with extension-> |{}| in directories:\n|\n|All size ({})\n|\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|-[{}]->[{} files]->[{}]\n|",
                    images_count,
                    
                    font_and_style(
                        FONT_STYLES.underline, 
                        FONT_COLORS.f_green, 
                        &extension
                    ),

                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow, 
                        &adapt_file_size(all_size)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Downloads dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.download.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow, 
                        &adapt_file_size(image_files_sizes_dirs.download)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Documents dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.document.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow, 
                        &adapt_file_size(image_files_sizes_dirs.document)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan,  
                        "Videos dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.video.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow, 
                        &adapt_file_size(image_files_sizes_dirs.video)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Pictures dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.picture.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow, 
                        &adapt_file_size(image_files_sizes_dirs.picture)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Local data dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.data_local.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow,  
                        &adapt_file_size(image_files_sizes_dirs.data_local)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Data dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.data.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow,  
                        &adapt_file_size(image_files_sizes_dirs.data)
                    ),

                    font_and_style(
                        FONT_STYLES.italic, 
                        FONT_COLORS.f_cyan, 
                        "Desktop dir"
                    ),
                    font_style(
                        FONT_STYLES.bold, 
                        &image_files_dirs_counts.desktop.to_string()
                    ),
                    font_and_style(
                        FONT_STYLES.bold, 
                        FONT_COLORS.f_yellow,  
                        &adapt_file_size(image_files_sizes_dirs.desktop)
                    ),
                )
            ); 
        } else if images_count == 0 {
            result.push_str(
                &format!(
                    "|[{}]-> Images with this extension |{}| not found in your computer!\n|[{}]-App scans only the main directories of your computer\n|-where photos may be stored.\n|",

                    full_style(
                        FONT_COLORS.f_white, 
                        BACKGROUND_COLORS.b_red, 
                        FONT_STYLES.bold, 
                        "Images not found!"
                    ),
                    font_and_style(
                        FONT_STYLES.underline, 
                        FONT_COLORS.f_green, 
                        &extension
                    ),
                    full_style(
                        FONT_COLORS.f_white, 
                        BACKGROUND_COLORS.b_blue, 
                        FONT_STYLES.bold, 
                        "Description"
                    )
                )
            );
        }    
    } else {
        result.push_str(
            &format!(
                "|[{}]->({})->This is not image!",

                full_style(
                    FONT_COLORS.f_white, 
                    BACKGROUND_COLORS.b_red, 
                    FONT_STYLES.bold, 
                    "Error!"
                ),
                font_style(
                    FONT_STYLES.underline, 
                    &extension
                )
            )
        );
    }   

    progress_bar.finish(); //finishing progress bar

    return result
}