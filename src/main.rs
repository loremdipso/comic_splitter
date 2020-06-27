use image::{GenericImage, Pixel};
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

// fn get_now() -> String {
// 	let now = chrono::Utc::now();
// 	return now.format("%S").to_string();
// }

fn split_image(output_dir: String, image_path: String) {
	// println!("{}", _output_dir);
	/* IDEA:
		Open image as pixels
		Print image
	*/

	let base_output_path = &Path::new(&output_dir).join(
		Path::new(&image_path)
			.file_name()
			.unwrap()
			.to_str()
			.unwrap(),
	);

	let original_buffer = image::open(image_path).ok().expect("Opening image failed");
	split_color_buffer(original_buffer, &base_output_path);

	// result.dy

	// image::DynamicImage::save("./output.jpg", image);
	// image::save_buffer(path, buf, width, height, color)

	// let image = (pixel_data, ImageInfo::rgb8(1920, 1080));

	// Create a window and display the image.
	// let window = make_window("image")?;
	// window.set_image(image, "image-001")?;
}

fn split_color_buffer(mut input_buffer: image::DynamicImage, base_output_path: &Path) {
	let grayscale_buffer = input_buffer.grayscale();
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

fn get_split_regions(buffer: image::DynamicImage) -> Vec<Region> {
	let mut regions = Vec::new();

	let width = 200;
	let height = 200;
	let x = 20;
	let y = 20;

	let region = Region {
		width,
		height,
		x,
		y,
	};

	regions.push(region);

	// // Iterate over the coordinates and pixels of the image
	// for (x, y, pixel) in buffer.enumerate_pixels_mut() {
	// 	let r = (0.3 * x as f32) as u8;
	// 	let b = (0.3 * y as f32) as u8;
	// 	*pixel = image::Rgb([r, 0, b]);
	// }

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
