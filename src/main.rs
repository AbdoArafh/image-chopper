use clap::Parser;
use progress_bar::*;
use std::{fs, path, process};

/// An app to chop images into smaller slices
#[derive(Parser)]
struct Args {
    /// path of the file to chop
    file_path: path::PathBuf,

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

            let file_path = args.file_path.to_string_lossy();
            let output_path = format!(
                "output/{}",
                &file_path[file_path.find("/").unwrap_or(0)
                    ..file_path.find(".").unwrap_or(file_path.len())]
            );
            fs::create_dir_all(&output_path).unwrap();

            let file_extension = &file_path[file_path.find(".").unwrap_or(0) + 1..];

            subimg
                .to_image()
                .save(format!(
                    "{}/{},{}.{}",
                    &output_path,
                    i + 1,
                    j + 1,
                    file_extension
                ))
                .unwrap_or_else(|_err| {
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
