use image::{DynamicImage, GenericImage, Luma};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    // Prints each argument on a separate line
    let mut todo = Vec::new();
    let mut output_dir: Option<String> = None;

    let mut it = env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--output" | "-output" => match it.next() {
                Some(dir) => {
                    output_dir = Some(dir);
                }
                None => {}
            },
            _ => {
                if Path::new(&arg).exists() {
                    todo.push(arg);
                }
            }
        }
    }

    if let Some(dir) = output_dir {
        for image in todo {
            let then = SystemTime::now();
            split_image(dir.clone(), image.clone());
            println!(
                "Finished in {}ms",
                SystemTime::now().duration_since(then).unwrap().as_millis()
            );
        }
    }
}

fn split_image(output_dir: String, image_path: String) {
    let base_output_path = &Path::new(&output_dir).join(
        Path::new(&image_path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let original_buffer = image::open(image_path).ok().expect("Opening image failed");
    split_color_buffer(original_buffer, &base_output_path);
}

fn split_color_buffer(mut input_buffer: DynamicImage, base_output_path: &Path) {
    let grayscale_buffer = input_buffer.to_luma();
    // let mut buffers = Vec::new();
    let locs = get_split_regions(grayscale_buffer);

    for (index, loc) in locs.iter().enumerate() {
        let sub_image = input_buffer
            .sub_image(loc.x, loc.y, loc.width, loc.height)
            .to_image();
        let output_path = add_file_suffix(base_output_path, index.to_string());
        sub_image.save(output_path).unwrap();
    }
}

fn get_split_regions(buffer: image::ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<Region> {
    let mut regions = Vec::new();

    let imgx = buffer.width();
    let imgy = buffer.height();
    let min_columns = 10;
    let mut column_count = 0;
    let mut current_y = 0;

    for y in 0..imgy {
        let mut row_is_white = true;
        for x in 0..imgx {
            let pixel = buffer.get_pixel(x, y).0[0];
            if pixel < 250 {
                row_is_white = false;
                break;
            }
            // let data = (*pixel as image::Rgb<u8>).0;
        }

        // TODO: add minimum slice size
        if row_is_white {
            column_count += 1;
        } else {
            if column_count >= min_columns {
                // doit
                let temp_y = y - (column_count / 2);

                let region = Region {
                    width: imgx,
                    height: temp_y - current_y,
                    x: 0,
                    y: current_y,
                };
                regions.push(region);

                current_y = temp_y;
            }
            column_count = 0;
        }
    }

    // add rest of image
    let region = Region {
        width: imgx,
        height: imgy - current_y,
        x: 0,
        y: current_y,
    };
    regions.push(region);

    return regions;
}

struct Region {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

// Adds the suffix in-between the path's name and the file extension
fn add_file_suffix(input_path: &Path, suffix: String) -> PathBuf {
    let new_filename = format!(
        "{}_{}.{}",
        input_path.file_stem().unwrap().to_str().unwrap(),
        suffix,
        input_path.extension().unwrap().to_str().unwrap()
    );

    return Path::new(input_path)
        .with_file_name(new_filename)
        .to_path_buf();
}
