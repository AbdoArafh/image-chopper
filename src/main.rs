use clap::Parser;
use progress_bar::*;
use std::{fs, path, process};

/// An app to chop images into smaller slices
#[derive(Parser)]
struct Args {
    /// path of the file to chop
    file_path: path::PathBuf,

    /// main output folder
    #[arg(short, long)]
    output_folder: Option<path::PathBuf>,

    /// number of horizontal slices
    x_slices: u8,

    /// number of vertical slices
    y_slices: u8,
}

fn main() {
    let args = Args::parse();
    let mut image = match image::open(&args.file_path) {
        Ok(img) => img,
        _ => {
            print_error("Error loading image");
            process::exit(1);
        }
    };

    let w = image.width();
    let h = image.height();

    let slices_count = (args.x_slices as u32, args.y_slices as u32);
    let slice_w = w / slices_count.0;
    let slice_h = h / slices_count.1;

    init_progress_bar((slices_count.0 * slices_count.1) as usize);
    set_progress_bar_action("Generating", Color::Blue, Style::Bold);

    use image::imageops;
    for i in 0..slices_count.0 {
        for j in 0..slices_count.1 {
            let subimg = imageops::crop(&mut image, i * slice_w, j * slice_h, slice_w, slice_h);

            // let file_path = args.file_path.to_string_lossy();
            let mut output_folder = match args.output_folder {
                Some(ref path) => path.to_owned(),
                None => path::PathBuf::from("output/"),
            };
            let file_name = args
                .file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            output_folder.push(&file_name[0..file_name.find('.').unwrap()]);
            fs::create_dir_all(&output_folder).unwrap();

            let file_extension = &args.file_path.extension().unwrap();

            let mut output_path = output_folder;
            output_path.push(format!(
                "{},{}.{}",
                i + 1,
                j + 1,
                file_extension.to_string_lossy()
            ));

            subimg.to_image().save(&output_path).unwrap_or_else(|_err| {
                print_error(format!("Error saving chunk {}, {}", i, j));
                process::exit(1);
            });

            inc_progress_bar();
        }
    }

    print_progress_bar_info("Saved", "Saved all files safely", Color::Green, Style::Bold);
    finalize_progress_bar();
}

fn print_error(message: impl std::fmt::Display) {
    eprintln!("{}", message);
}

fn find_last(str: impl std::fmt::Display, c: char) -> Option<usize> {
    let str = str.to_string();
    let len = str.len();
    for (i, _c) in str.chars().rev().enumerate() {
        if c == _c {
            return Some(len - i);
        }
    }
    None
}
