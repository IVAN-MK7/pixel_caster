use image::{DynamicImage, ImageBuffer, Rgb, Rgba};
use serde::{Deserialize, Serialize};

use crate::bgra_management::{
    u32_bytes_oredered_indexes_and_fullvalues, ColorAlteration, SwitchBytes,
};

/// BGRA for the invisible pixels (those to not display, Alpha = 0). B=G=R=A=0 combination stands for completely transparent black
pub const BGRA_INVISIBLE_PIXEL: (u8, u8, u8, u8) = (0, 0, 0, 0);

/// Permits only to Vec\<u8> and Vec\<u32> to be used
pub trait PixelValues<T> {
    /// Adjusts pixels' Alpha values for when they must be sent with Alpha channel enabled,
    /// this will make so that each pixel's transparency is adjusted to the value the Windows API needs to display it with the wanted transparency.
    /// The Windows API use premultiplied alpha, which means that the Red, Green and Blue channel values must be premultiplied with the Alpha channel value.
    /// For example, if the alpha channel value is x, the Red, Green and Blue channels must be multiplied by x and divided by 0xff (255) prior to the call.
    fn create_adjusted_vec(vec: &[T]) -> Vec<T>;

    /// Creates a Vec of the given type and sets its capacity and length needed to contain the pixels' color values of the of the given area (u8 : area_width * area_height * 4, u32 : area_width * area_height)
    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<T>;

    /// How many units of this type of value are necessary to represent a single pixel's color (u8 : 4 (1Blue,1Red,1Green,1Alpha), u32 : 1 (0xAARRGGBB))
    fn units_per_pixel() -> u8;

    /// Returns .units_per_pixel() for the type specified at &self
    fn get_units_per_pixel(&self) -> u8;
}
impl PixelValues<u8> for u8 {
    fn create_adjusted_vec(vec: &[u8]) -> Vec<u8> {
        let mut vec_adjusted: Vec<u8> = Vec::with_capacity(vec.len());

        let mut i = 0;
        for _ in 0..vec.len() / 4 {
            // get Alpha Red Green Blue values
            let blue = vec[i];
            let green = vec[i + 1];
            let red = vec[i + 2];
            let alpha = vec[i + 3];
            let diff = 255 - alpha;
            let mut blue_adjusted = blue;
            let mut green_adjusted = green;
            let mut red_adjusted = red;
            // in case the current BGRA value's Alpha is not at it's max (255 (u8))
            // we need to adjust the values of the BGR bytes
            if diff > 0 {
                if blue > 255 - diff {
                    blue_adjusted = 255 - diff;
                }
                if green > 255 - diff {
                    green_adjusted = 255 - diff;
                }
                if red > 255 - diff {
                    red_adjusted = 255 - diff;
                }
            }
            vec_adjusted.extend_from_slice(&[blue_adjusted, green_adjusted, red_adjusted, alpha]);

            i += 4;
        }
        vec_adjusted
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u8> {
        vec![0; area_width * area_height * <u8>::units_per_pixel() as usize]
    }

    fn units_per_pixel() -> u8 {
        4
    }
    fn get_units_per_pixel(&self) -> u8 {
        4
    }
}
impl PixelValues<u32> for u32 {
    fn create_adjusted_vec(vec: &[u32]) -> Vec<u32> {
        let mut vec_adjusted: Vec<u32> = Vec::with_capacity(vec.len());

        let (ordered_indexes, fullvalues) = u32_bytes_oredered_indexes_and_fullvalues();
        let v1_index = ordered_indexes[0];
        let v2_index = ordered_indexes[1];
        let v3_index = ordered_indexes[2];
        let v4_index = ordered_indexes[3];
        let v1_full_val = fullvalues[0];
        let v2_full_val = fullvalues[1];
        let v3_full_val = fullvalues[2];
        let v4_full_val = fullvalues[3];

        for p in vec {
            // get Blue Green Red Alpha values by excluding the others
            // for each make null the bytes that are not those representing the value we need
            // e.g., Alpha value : is the 4th in BGRA (in a CPU with little endianness is represented by the 2 most left bytes), so with "0xFF00_0000" we keep them,
            // shift them enough position to have them at the most right so that they will be represented in a 0-255 range of values,
            // and assign to the variable "alpha"
            let blue: u32 = (p & v1_full_val) >> v1_index;
            let green: u32 = (p & v2_full_val) >> v2_index;
            let red: u32 = (p & v3_full_val) >> v3_index;
            let alpha: u32 = (p & v4_full_val) >> v4_index;
            let diff: u32 = 255 - alpha;
            let mut red_adjusted: u32 = red;
            let mut green_adjusted: u32 = green;
            let mut blue_adjusted: u32 = blue;
            // in case the current BGRA value's Alpha is not at it's max (255 (u8), FF00 0000 (HEX), 4.278.190.080 (u32))
            // we need to adjust the values of the BGR bytes
            if diff > 0 {
                if red > 255 - diff {
                    red_adjusted = 255 - diff;
                }
                if green > 255 - diff {
                    green_adjusted = 255 - diff;
                }
                if blue > 255 - diff {
                    blue_adjusted = 255 - diff;
                }
            }
            // set the BGRA values back to their u32 original positions (in a CPU with little endianness BGRA bytes will go from right to left : 0xAARRGGBB)
            vec_adjusted.extend_from_slice(&[(alpha << v4_index)
                + (red_adjusted << v3_index)
                + (green_adjusted << v2_index)
                + (blue_adjusted << v1_index)]);
        }
        vec_adjusted
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u32> {
        vec![0; area_width * area_height * <u32>::units_per_pixel() as usize]
    }

    fn units_per_pixel() -> u8 {
        1
    }
    fn get_units_per_pixel(&self) -> u8 {
        1
    }
}

/// Contains pixels' color bytes data, in BGRA format, and info
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PixelsCollection<T: PixelValues<T>> {
    /// Width of the rectangle represented by the color bytes
    pub width: usize,
    /// Height of the rectangle represented by the color bytes
    pub height: usize,
    /// Units of color bytes necessary to build a single row of pixels in the rectangle of pixels
    /// To consider "(y * image.width + x) * units_per_pixel()" instead of "image.row_length * y + 4 * x", it removes the need for this field.
    pub row_length: usize,
    /// Vec of bytes containing pixels' color data
    pub bytes: Vec<T>,
    /// How many units of this type of value are necessary to represent a single pixel's color (u8 : 4 (1Blue,1Red,1Green,1Alpha), u32 : 1 (0xAARRGGBB))
    pub units_per_pixel: u8,
}
impl<T: PixelValues<T>> PixelsCollection<T> {
    /// Creates a new instance that will represent a rectangle with width * height area, filled with the provided color bytes
    pub fn create(
        width: usize,
        height: usize,
        bytes: Vec<T>,
    ) -> Result<PixelsCollection<T>, String> {
        // if bytes.len()%4 != 0
        if bytes.len() % <T>::units_per_pixel() as usize != 0 {
            return Err("provided Vec<u8>'s length must be divisible by 4, as it takes 4 values (BGRA, in Vec<u8>) to get the resulting color for each pixel".to_string());
        }
        if bytes.len() != width * height * <T>::units_per_pixel() as usize {
            return Err(
                "provided Vec's length does not match width * height of the resulting values area"
                    .to_string(),
            );
        }
        Ok(PixelsCollection {
            width,
            height,
            row_length: ((width * height * <T>::units_per_pixel() as usize) / height),
            bytes,
            units_per_pixel: <T>::units_per_pixel(),
        })
    }
}

impl PixelsCollection<u8> {
    pub fn switch_bytes(&mut self, i1: usize, i2: usize) {
        <u8>::switch_bytes(&mut self.bytes, i1, i2);
    }
    /// If a BGRA combination is met, set it to a provided BGRA
    pub fn matching_color_change(
        &mut self,
        b: u8,
        g: u8,
        r: u8,
        a: u8,
        new_b: u8,
        new_g: u8,
        new_r: u8,
        new_a: u8,
    ) {
        self.bytes.color_matcher_and_new_color(
            |v0: u8, v1: u8, v2: u8, v3: u8| -> bool { v0 == b && v1 == g && v2 == r && v3 == a },
            new_b,
            new_g,
            new_r,
            new_a,
        );
    }
    /// Sets BGRA for the invisible pixels (not displayed, Alpha = 0, which BGRA values match BGRA_INVISIBLE_PIXEL)
    pub fn set_bgra_for_invisible(&mut self, b: u8, g: u8, r: u8, a: u8) {
        self.bytes.color_matcher_and_new_color(
            |v0: u8, v1: u8, v2: u8, v3: u8| -> bool {
                v0 == BGRA_INVISIBLE_PIXEL.0
                    && v1 == BGRA_INVISIBLE_PIXEL.1
                    && v2 == BGRA_INVISIBLE_PIXEL.2
                    && v3 == BGRA_INVISIBLE_PIXEL.3
            },
            b,
            g,
            r,
            a,
        );
    }
    /// Sets BGR values for every pixel
    pub fn set_bgr(&mut self, b: u8, g: u8, r: u8) {
        self.bytes.set_bgr(b, g, r);
    }

    /// If a BGR combination of any grey (equal B,G,R make shades of grey) is met and their value exceed the given threshold,
    /// it's Alpha will be set to a provided value. Every grey will be set to black
    pub fn grey_scale_into_black(vec: &mut [u8], grey_threshold: u8) {
        let mut i = 0;
        for _ in 0..(vec.len() / 4) {
            // sets greys (equal B,G,R make shades of grey) opacity to a given Alpha when they come too close to white (B,G,R : 255)
            // / too far away from black (B,G,R : 0) by a given grey_threshold value
            // >137 , >149 ...
            // shade of grey
            if vec[i] == vec[i + 1] && vec[i] == vec[i + 2] {
                if vec[i] > grey_threshold {
                    vec[i] = BGRA_INVISIBLE_PIXEL.0;
                    vec[i + 1] = BGRA_INVISIBLE_PIXEL.1;
                    vec[i + 2] = BGRA_INVISIBLE_PIXEL.2;
                    vec[i + 3] = BGRA_INVISIBLE_PIXEL.3;
                }
                // transforms shades of grey into fully opaque black, except for white
                else {
                    vec[i] = 0;
                    vec[i + 1] = 0;
                    vec[i + 2] = 0;
                    vec[i + 3] = 255;
                }
            }
            i += 4;
        }
    }
    /// Whites become transparent, range from white to lowest BGR will get a proportionate Alpha value. Where Alpha < 255 no changes will be made (the values of colors with transparency won't be alterated)
    pub fn white_background_to_transparency_gradient(vec: &[u8]) -> Vec<u8> {
        let mut vec_adjusted: Vec<u8> = Vec::with_capacity(vec.len());

        let mut j = 0;
        // get Alpha Rred Green Blue values
        let mut lowest_blue = 255;
        let mut lowest_green = 255;
        let mut lowest_red = 255;
        for _ in 0..vec.len() / 4 {
            if vec[j] < lowest_blue {
                lowest_blue = vec[j];
            }
            if vec[j + 1] < lowest_green {
                lowest_green = vec[j + 1];
            }
            if vec[j + 2] < lowest_red {
                lowest_red = vec[j + 2];
            }
            j += 4;
        }
        let mut lowest_val = 255;
        let mut lowest_val_index = 0;
        if lowest_blue < lowest_val {
            lowest_val = lowest_blue;
            lowest_val_index = 0;
        }
        if lowest_green < lowest_val {
            lowest_val = lowest_green;
            lowest_val_index = 1;
        }
        if lowest_red < lowest_val {
            lowest_val = lowest_red;
            lowest_val_index = 2;
        }

        let mut i = 0;
        for _ in 0..vec.len() / 4 {
            // get Alpha Rred Green Blue values
            let blue = vec[i];
            let green = vec[i + 1];
            let red = vec[i + 2];
            let alpha = vec[i + 3];
            // if pixel has full opacity (no transparency, Alpha is at it's max : 255 (u8))
            if alpha == 255 {
                // and pixel is not white (B=G=R == 255)
                if blue < 255 || green < 255 || red < 255 {
                    // we need to adjust the values of the RGB bytes

                    // get the byte we need as adjusting base (this color's byte which in the whole image reached the lowest value (B or G or R))
                    let this_val = vec[i + lowest_val_index];
                    let alpha_adjusted = 255 - this_val + lowest_val;
                    vec_adjusted.extend_from_slice(&[blue, green, red, alpha_adjusted]);
                }
                // pixel is white, so make it transparent. add lowest BGR values to keep the bytes all with the same BGR values, and only change the Alpha
                else {
                    vec_adjusted.extend_from_slice(&[blue, green, red, 0]);
                }
            }
            // pixel has some transparency (Alpha < 255), leave it as is
            else {
                vec_adjusted.extend_from_slice(&[blue, green, red, alpha]);
            }

            i += 4;
        }
        vec_adjusted
    }
    /// Blacks become transparent, range from black to highest BGR will get a proportionate Alpha value. Where Alpha < 255 no changes will be made (the values of colors with transparency won't be alterated)
    pub fn black_background_to_transparency_gradient(vec: &[u8]) -> Vec<u8> {
        let mut vec_adjusted: Vec<u8> = Vec::with_capacity(vec.len());

        let mut j = 0;
        // get Alpha Rred Green Blue values
        let mut highest_blue = 0;
        let mut highest_green = 0;
        let mut highest_red = 0;
        for _ in 0..vec.len() / 4 {
            if vec[j] > highest_blue {
                highest_blue = vec[j];
            }
            if vec[j + 1] > highest_green {
                highest_green = vec[j + 1];
            }
            if vec[j + 2] > highest_red {
                highest_red = vec[j + 2];
            }
            j += 4;
        }
        let mut highest_val = 0;
        let mut highest_val_index = 0;
        if highest_blue > highest_val {
            highest_val = highest_blue;
            highest_val_index = 0;
        }
        if highest_green > highest_val {
            highest_val = highest_green;
            highest_val_index = 1;
        }
        if highest_red > highest_val {
            highest_val = highest_red;
            highest_val_index = 2;
        }

        let mut i = 0;
        for _ in 0..vec.len() / 4 {
            // get Alpha Rred Green Blue values
            let blue = vec[i];
            let green = vec[i + 1];
            let red = vec[i + 2];
            let alpha = vec[i + 3];
            // if pixel has full opacity (no transparency, Alpha is at it's max : 255 (u8))
            if alpha == 255 {
                // and pixel is not black (B=G=R == 0)
                if blue > 0 || green > 0 || red > 0 {
                    // we need to adjust the values of the RGB bytes

                    // get the byte we need as adjusting base (this color's byte which in the whole image reached the highest value (B or G or R))
                    let this_val = vec[i + highest_val_index];
                    let alpha_adjusted = highest_val - (highest_val - this_val);
                    vec_adjusted.extend_from_slice(&[blue, green, red, alpha_adjusted]);
                }
                // pixel is black, so make it transparent. add highest BGR values to keep the bytes all with the same BGR values, and only change the Alpha
                else {
                    vec_adjusted.extend_from_slice(&[blue, green, red, 0]);
                }
            }
            // pixel has some transparency (Alpha < 255), leave it as is
            else {
                vec_adjusted.extend_from_slice(&[blue, green, red, alpha]);
            }

            i += 4;
        }
        vec_adjusted
    }

    /// Returns a new PixelsCollection from the provided one, scaling based on the provided ResizeSize (either width or hight)
    pub fn create_new_resized(&self, resize_size: ResizeSize) -> PixelsCollection<u8> {
        let new_height;
        let new_width;

        match resize_size {
            ResizeSize::Width(w) => {
                new_width = w;
                new_height = (new_width * self.height) / self.width;
            }
            ResizeSize::Height(h) => {
                new_height = h;
                new_width = (new_height * self.width) / self.height;
            }
        }

        let x_ratio: f32 = self.width as f32 / new_width as f32;
        let y_ratio: f32 = self.height as f32 / new_height as f32;

        let mut bytes_resized = Vec::<u8>::with_capacity(new_width * new_height * 4);
        for fy in 0..new_height {
            for fx in 0..new_width {
                let pixel_needed = self.row_length * (fy as f32 * y_ratio) as usize
                    + 4 * (fx as f32 * x_ratio) as usize;
                bytes_resized.push(self.bytes[pixel_needed]);
                bytes_resized.push(self.bytes[pixel_needed + 1]);
                bytes_resized.push(self.bytes[pixel_needed + 2]);
                bytes_resized.push(self.bytes[pixel_needed + 3]);
            }
        }

        PixelsCollection::create(new_width, new_height, bytes_resized).unwrap()
    }
}


impl<T: PixelValues<T>> PixelsCollection<T> {
    pub fn coord_to_index(&self, x: usize, y: usize) -> usize {
        (self.width * y + x) * (self.units_per_pixel as usize)
    }

    /// From the given `x` `y` coordinate on its `bytes`' image, applies the provided offset values
    /// and returns the resulting coordinate's index.
    pub fn coord_to_index_with_offset_coord(&self, x: usize, y: usize, offset_x: isize, offset_y: isize) -> usize {
        ((self.width as isize * (y as isize + offset_y) + (x as isize + offset_x)) * (self.units_per_pixel as isize)) as usize
    }
}

/// Measure of either Width or Height
pub enum ResizeSize {
    Width(usize),
    Height(usize),
}

impl PixelsCollection<u32> {
    pub fn switch_bytes(&mut self, i1: usize, i2: usize) {
        <u32>::switch_bytes(&mut self.bytes, i1, i2);
    }
}

/// Note: this function assumes that the provided `image_data` is already in BGR(A) format,
/// if it's not switching the Red and Blue bytes with `<u8>::switch_bytes(&mut pixels_collection.bytes, 0, 2)`.
pub fn dynamic_image_data_to_pixels_collection(image_data: Vec<u8>, has_alpha_channel: bool) -> Result<PixelsCollection::<u8>, String> {
    // Load the image from the png data
    let image = match image::load_from_memory(&image_data) {
        Ok(img) => img,
        Err(e) => return Err(format!("Error loading image: {}", e)),
    };
    dynamic_image_to_pixels_collection(image, has_alpha_channel)
}
/// Note: this function assumes that the provided `image` is already in BGR(A) format,
/// if it's not switching the Red and Blue bytes with `<u8>::switch_bytes(&mut pixels_collection.bytes, 0, 2)`.
pub fn dynamic_image_to_pixels_collection(image: DynamicImage, has_alpha_channel: bool) -> Result<PixelsCollection::<u8>, String> {
    // Convert the image to BGR(A)
    let (bgra_bytes, width, height) = if has_alpha_channel {
        let bgr_image = image.to_rgba8();
        let (w, h) = bgr_image.dimensions();
        (bgr_image.into_raw(), w, h)
    } else {
        println!("Creating BGRA vec");
        let bgr_image = image.to_rgb8();
        let (w, h) = bgr_image.dimensions();
        let bgr_bytes = bgr_image.into_raw();
        let mut bgra_bytes = vec![];
        bgr_bytes.chunks_exact(3).for_each(|c| {
            bgra_bytes.extend_from_slice(c);
            bgra_bytes.push(255);
        });
        (bgra_bytes, w, h)
    };

    Ok(PixelsCollection::<u8>::create(width as usize, height as usize, bgra_bytes).unwrap())
}
pub fn pixels_collection_to_dynamic_image_data(pixels_collection: PixelsCollection::<u8>, keep_alpha_channel: bool) -> Result<Vec::<u8>, String> {
    use std::io::Cursor;

    // Convert BGRA to RGBA
    /*let mut rgba_data = Vec::with_capacity(pixels_collection.bytes.len());
    for chunk in pixels_collection.bytes.chunks_exact(4) {
        rgba_data.push(chunk[2]); // R
        rgba_data.push(chunk[1]); // G
        rgba_data.push(chunk[0]); // B
        rgba_data.push(chunk[3]); // A
    }*/

    let mut png_data = Vec::new();
    // Create an ImageBuffer from the BGR(A) data
    if keep_alpha_channel {
        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(pixels_collection.width as u32, pixels_collection.height as u32, pixels_collection.bytes)
            .ok_or("Failed to create image buffer")?;
        img.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png).map_err(|e| e.to_string())?;
    } else {
        let mut vec = vec![];
        pixels_collection.bytes.chunks_exact(4).for_each(|c| vec.extend_from_slice(&c[0..3]));

        let img = ImageBuffer::<Rgb<u8>, _>::from_raw(pixels_collection.width as u32, pixels_collection.height as u32, vec)
            .ok_or("Failed to create image buffer")?;
        img.write_to(&mut Cursor::new(&mut png_data), image::ImageFormat::Png).map_err(|e| e.to_string())?;
    };
    Ok(png_data)
}
