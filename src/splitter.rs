use image::{DynamicImage, GenericImage, Luma};
use std::collections::HashMap;
use std::fs;
use std::ops::Sub;
use std::path::Path;
use std::path::PathBuf;

type Buffer = image::ImageBuffer<Luma<u8>, Vec<u8>>;

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
		let mut regions = Vec::new();

		let imgx = buffer.width();
		let imgy = buffer.height();
		let bad_pixel_limit = imgx / 20; // 1 of every 20 pixels can be bad
		let start_y = imgy / 5; // start 1/5 of the way down the image
		let end_y = imgy - start_y; // end 4/5 of the way down the image

		let min_white_rows = 10;
		let min_height = imgy / 5;

		let mut consecutive_empty_rows = 0;
		let mut current_y = 0;

		let (goal, error_margin) = if self.black_lines { (0, 100) } else { (255, 5) };

		for y in start_y..end_y {
			let mut num_bad_pixels = 0;
			let mut row_is_empty = true;
			for x in 0..imgx {
				// TODO: measure variation
				let pixel = buffer.get_pixel(x, y).0[0];

				if Splitter::abs_difference(pixel, goal) > error_margin {
					num_bad_pixels += 1;
					if num_bad_pixels > bad_pixel_limit {
						row_is_empty = false;
						break;
					}
				}
			}

			// TODO: add minimum slice size
			if row_is_empty {
				consecutive_empty_rows += 1;
			} else {
				if consecutive_empty_rows >= min_white_rows {
					let temp_y = y - (consecutive_empty_rows / 2);
					let height = temp_y - current_y;
					if height > min_height {
						let region = Region {
							width: imgx,
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

		// add rest of image, but only if we did any splitting to begin with
		if regions.len() == 0 {
			println!("Found no regions, writing whole image for: {:?}", &path);
			let region = Region {
				width: imgx,
				height: imgy - current_y,
				x: 0,
				y: current_y,
			};
			regions.push(region);
		} else if regions.len() > 0 {
			let region = Region {
				width: imgx,
				height: imgy - current_y,
				x: 0,
				y: current_y,
			};
			regions.push(region);
		}

		return regions;
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