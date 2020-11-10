use image::{DynamicImage, GenericImage, Luma};
use std::fs;
use std::ops::Sub;
use std::path::Path;
use std::path::PathBuf;

type Buffer = image::ImageBuffer<Luma<u8>, Vec<u8>>;

#[derive(Debug, Clone)]
struct Region {
	width: u32,
	height: u32,
	x: u32,
	y: u32,
}

pub struct Splitter {
	pub remove_original: bool,
	pub black_lines: bool,
}

impl Splitter {
	pub fn split_image(self: &Splitter, output_dir: PathBuf, image_path: PathBuf) {
		let base_output_path = &Path::new(&output_dir).join(
			Path::new(&image_path)
				.file_name()
				.expect(&format!("{:?} doesn't exist", image_path))
				.to_str()
				.unwrap(),
		);

		let original_buffer = image::open(image_path.clone())
			.ok()
			.expect("Opening image failed");

		let num_split = self.split_color_buffer(original_buffer, &base_output_path);
		if num_split > 0 {
			if self.remove_original {
				println!("\tRemoving {:?}...", image_path);
				fs::remove_file(Path::new(&image_path))
					.ok()
					.expect("ERROR DELETING FILE");
			}
		}
	}

	fn split_color_buffer(
		self: &Splitter,
		mut input_buffer: DynamicImage,
		base_output_path: &Path,
	) -> usize {
		let grayscale_buffer = input_buffer.to_luma();
		let locs = self.get_split_regions(grayscale_buffer, &base_output_path);

		for (index, loc) in locs.iter().enumerate() {
			let sub_image = input_buffer
				.sub_image(loc.x, loc.y, loc.width, loc.height)
				.to_image();
			let output_path = Splitter::add_file_suffix(base_output_path, index.to_string());
			dbg!(&output_path);
			sub_image
				.save(&output_path)
				.expect(&format!("{:?} ", &output_path.as_os_str()));
		}

		return locs.len();
	}

	fn get_split_regions(self: &Splitter, buffer: Buffer, path: &Path) -> Vec<Region> {
		// let img_x = buffer.width();
		// let img_y = buffer.height();
		// let start_y = img_y / 5; // start 1/5 of the way down the image
		// let end_y = img_y - start_y; // end 4/5 of the way down the image

		return self.get_split_regions_recursive(
			&buffer,
			&Region {
				x: 0,
				y: 0,
				width: buffer.width(),
				height: buffer.height(),
			},
		);
	}

	fn get_split_regions_recursive(
		self: &Splitter,
		buffer: &Buffer,
		section: &Region,
	) -> Vec<Region> {
		let mut regions = Vec::new();
		let (goal_color, error_margin) = if self.black_lines { (0, 100) } else { (255, 5) };

		// let mut angles = Vec::new();
		// let mut thing = -4.;
		// while thing < 4. {
		// 	thing += 0.1;
		// 	angles.push(thing);
		// }
		let angles = [
			0., -0.1, 0.1, -0.6, -0.5, -0.4, -0.3, -0.2, 0.2, 0.3, 0.4, 0.5, 0.6,
		];

		// TODO: same thing, but vertical
		let mut angle_index = 0;
		while regions.len() == 0 && angle_index < angles.len() {
			let angle = angles[angle_index];
			let min_empty_rows = 5;
			let min_height = buffer.height() / 5;
			let mut consecutive_empty_rows = 0;
			let mut current_y = section.y;

			for y in (section.y)..(section.y + section.height) {
				let line = Splitter::get_line(&buffer, &section, y, angle);
				if Splitter::line_is_single_color(line, goal_color, error_margin) {
					consecutive_empty_rows += 1;
				} else {
					if consecutive_empty_rows >= min_empty_rows {
						// dbg!(angle);
						let temp_y = y - (consecutive_empty_rows / 2);
						let temp_y =
							Splitter::get_y(temp_y, (section.x + section.width) / 2, angle) as u32;
						let height = temp_y - current_y;
						if height > min_height {
							let region = Region {
								width: section.width,
								height: temp_y - current_y,
								x: 0,
								y: current_y,
							};
							regions.push(region);

							current_y = temp_y;
						}
					}

					consecutive_empty_rows = 0;
				}
			}

			if regions.len() > 0 {
				// add rest of image, but only if we did any splitting to begin with
				let region = Region {
					width: section.width,
					height: (section.height + section.y) - current_y,
					x: section.x,
					y: current_y,
				};
				regions.push(region);
			}

			angle_index += 1;
		}

		// add entire section if we did no splitting
		if regions.len() == 0 {
			regions.push(section.clone());
			return regions;
		} else {
			let mut new_regions = Vec::new();
			for region in &regions {
				new_regions.append(&mut self.get_split_regions_recursive(&buffer, &region));
			}
			return new_regions;
		}
	}

	fn line_is_single_color(line: Vec<u8>, goal_color: usize, error_margin: usize) -> bool {
		let mut total: usize = 0;
		let mut count: usize = 0;

		if line.len() == 0 {
			return false;
		}

		for x in &line {
			total += usize::from(*x);
			count += 1;
			if count % 10 == 0 {
				let average_pixel: usize = total / count;
				if Splitter::abs_difference(average_pixel, goal_color) > error_margin {
					return false;
				}
			}
		}

		true
	}

	fn get_line(buffer: &Buffer, section: &Region, start_y: u32, angle: f64) -> Vec<u8> {
		let mut rv = Vec::new();

		let mut x = section.x;
		while x < section.x + section.width {
			let ty = Splitter::get_y(start_y, x, angle);
			if ty < 0 {
				return Vec::new();
			}

			let ty = ty as u32;
			if ty < (section.y + section.height) {
				let pixel = buffer.get_pixel(x, ty).0[0];
				rv.push(pixel);
			} else {
				return Vec::new();
			}
			x += 1;
		}

		return rv;
	}

	fn get_y(start_y: u32, x: u32, angle: f64) -> i64 {
		return (start_y as f64 + angle * x as f64) as i64;
	}

	fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
		if x < y {
			y - x
		} else {
			x - y
		}
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
}
