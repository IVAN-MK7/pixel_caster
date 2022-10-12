use crate::bgra_management::{u32_bytes_oredered_indexes_and_fullvalues, ColorAlteration, SwitchBytes};

/// BGRA for the invisible pixels (those to not display, Alpha = 0). B=G=R=A=0 combination stands for completely transparent black
pub const BGRA_INVISIBLE_PIXEL :(u8,u8,u8,u8) = (0,0,0,0);

/// Permits only to Vec\<u8> and Vec\<u32> to be used
pub trait PixelValues<T> {
    /// Adjusts pixels' Alpha values for when they must be sent with Alpha channel enabled,
    /// this will make so that each pixel's transparency is adjusted to the value the Windows API needs to display it with the wanted transparency.
    /// The Windows API use premultiplied alpha, which means that the Red, Green and Blue channel values must be premultiplied with the Alpha channel value.
    /// For example, if the alpha channel value is x, the Red, Green and Blue channels must be multiplied by x and divided by 0xff (255) prior to the call.
    fn create_adjusted_vec(vec :&Vec<T>) -> Vec<T>;

    /// Creates a Vec of the given type and sets its capacity and length needed to contain the pixels' color values of the of the given area (u8 : area_width * area_height * 4, u32 : area_width * area_height)
    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<T>;

    /// How many units of this type of value are necessary to represent a single pixel's color (u8 : 4 (1Blue,1Red,1Green,1Alpha), u32 : 1 (0xAARRGGBB))
    fn units_per_pixel() -> u8;

    /// Returns .units_per_pixel() for the type specified at &self
    fn get_units_per_pixel(&self) -> u8;
}
impl PixelValues<u8> for u8 {
    fn create_adjusted_vec(vec :&Vec<u8>) -> Vec<u8> {
        let mut vec_adjusted :Vec<u8> = Vec::with_capacity(vec.len());
    
        let mut i = 0;
        for _ in 0..vec.len()/4 {
            // get Alpha Red Green Blue values
            let blue =  vec[i];
            let green = vec[i+1];
            let red = vec[i+2];
            let alpha = vec[i+3];
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
        return vec_adjusted;
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u8> {
        let mut vec_u8_size_set = Vec::<u8>::with_capacity(area_width * area_height * <u8>::units_per_pixel() as usize);
        unsafe { vec_u8_size_set.set_len(vec_u8_size_set.capacity()); }
        return vec_u8_size_set
    }

    fn units_per_pixel() -> u8 { 4 }
    fn get_units_per_pixel(&self) -> u8 { 4 }
}
impl PixelValues<u32> for u32 {
    fn create_adjusted_vec(vec :&Vec<u32>) -> Vec<u32> {
        let mut vec_adjusted :Vec<u32> = Vec::with_capacity(vec.len());
        
        let (ordered_indexes, fullvalues)= u32_bytes_oredered_indexes_and_fullvalues();
        let v1_index = ordered_indexes[0];
        let v2_index = ordered_indexes[1];
        let v3_index = ordered_indexes[2];
        let v4_index = ordered_indexes[3];
        let v1_full_val = fullvalues[0];
        let v2_full_val = fullvalues[1];
        let v3_full_val = fullvalues[2];
        let v4_full_val = fullvalues[3];
    
        for i in 0..vec.len() {
    
            // get Blue Green Red Alpha values by excluding the others
            // for each make null the bytes that are not those representing the value we need
            // e.g., Alpha value : is the 4th in BGRA (in a CPU with little endianness is represented by the 2 most left bytes), so with "0xFF00_0000" we keep them,
            // shift them enough position to have them at the most right so that they will be represented in a 0-255 range of values,
            // and assign to the variable "alpha"
            let blue :u32 = (vec[i] & v1_full_val) >> v1_index;
            let green :u32 = (vec[i] & v2_full_val) >> v2_index;
            let red :u32 = (vec[i] & v3_full_val) >> v3_index;
            let alpha :u32 = (vec[i] & v4_full_val) >> v4_index;
            let diff :u32 = 255 - alpha;
            let mut red_adjusted :u32 = red;
            let mut green_adjusted :u32 = green;
            let mut blue_adjusted :u32 = blue;
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
            vec_adjusted.extend_from_slice(&[(alpha << v4_index) + (red_adjusted << v3_index) + (green_adjusted << v2_index) + (blue_adjusted << v1_index)]);
        }
        return vec_adjusted;
    }

    fn initialize_vec(area_width: usize, area_height: usize) -> Vec<u32> {
        let mut vec_u32_size_set = Vec::<u32>::with_capacity(area_width * area_height * <u32>::units_per_pixel() as usize);
        unsafe { vec_u32_size_set.set_len(vec_u32_size_set.capacity()); }
        return vec_u32_size_set
    }

    fn units_per_pixel() -> u8 { 1 }
    fn get_units_per_pixel(&self) -> u8 { 1 }
}


/// Contains pixels' color bytes data and info
#[derive(Clone)]
pub struct PixelsCollection<T: PixelValues<T>> {
    /// Width of the rectangle represented by the color bytes
    pub width :usize,
    /// Height of the rectangle represented by the color bytes
    pub height :usize,
    /// Units of color bytes necessary to build a single row of pixels in the rectangle of pixels
    pub row_length :usize,
    /// Vec of bytes containing pixels' color data
    pub bytes :Vec<T>,
    /// How many units of this type of value are necessary to represent a single pixel's color (u8 : 4 (1Blue,1Red,1Green,1Alpha), u32 : 1 (0xAARRGGBB))
    pub units_per_pixel: u8,
}
impl<T: PixelValues<T>> PixelsCollection<T> {
    /// Creates a new instance that will represent a rectangle with width * height area, filled with the provided color bytes
    pub fn create(width :usize, height :usize, bytes :Vec<T>) -> Result<PixelsCollection<T>, String> {
        // if bytes.len()%4 != 0
        if bytes.len() % <T>::units_per_pixel() as usize != 0 {
            return Err("provided Vec<u8>'s length must be divisible by 4, as it takes 4 values (BGRA, in Vec<u8>) to get the resulting color for each pixel".to_string());
        }
        if bytes.len() != width * height * <T>::units_per_pixel() as usize {
            return Err("provided Vec's length does not match width * height of the resulting values area".to_string());
        }
        Ok(PixelsCollection {
            width,
            height,
            row_length: ((width * height * <T>::units_per_pixel() as usize) / height) as usize,
            bytes,
            units_per_pixel: <T>::units_per_pixel()
        })
    }
}

impl PixelsCollection<u8> {
    pub fn switch_bytes(&mut self, i1 :usize, i2 :usize) {
        <u8>::switch_bytes(&mut self.bytes, i1, i2);
    }
    /// If a BGRA combination is met, set it to a provided BGRA
    pub fn matching_color_change (&mut self, b :u8, g :u8, r :u8, a : u8, new_b :u8, new_g :u8, new_r :u8, new_a :u8) {
        self.bytes.color_matcher_and_new_color(|v0:u8,v1:u8,v2:u8,v3:u8| -> bool { v0 == b && v1 == g && v2 == r && v3 == a},new_b,new_g,new_r,new_a);
    }
    /// Sets BGRA for the invisible pixels (not displayed, Alpha = 0, which BGRA values match BGRA_INVISIBLE_PIXEL)
    pub fn set_bgra_for_invisible (&mut self, b :u8, g :u8, r :u8, a : u8) {
        self.bytes.color_matcher_and_new_color(
            |v0:u8,v1:u8,v2:u8,v3:u8| -> bool {
                v0 == BGRA_INVISIBLE_PIXEL.0 &&
                v1 == BGRA_INVISIBLE_PIXEL.1 &&
                v2 == BGRA_INVISIBLE_PIXEL.2 &&
                v3 == BGRA_INVISIBLE_PIXEL.3
            },
            b,g,r,a
        );
    }
    /// If a BGR combination of any grey (equal B,G,R make shades of grey) is met and their value exceed the given threshold, 
    /// it's Alpha will be set to a provided value. Every grey will be set to black
    pub fn grey_scale_into_black (vec : &mut Vec<u8>, grey_threshold :u8) {
        let mut i = 0;
        for _ in 0..(vec.len()/4) {
            // sets greys (equal B,G,R make shades of grey) opacity to a given Alpha when they come too close to white (B,G,R : 255)
            // / too far away from black (B,G,R : 0) by a given grey_threshold value
            // >137 , >149 ...
            // shade of grey
            if vec[i] == vec[i+1] && vec[i] == vec[i+2] {
                if  vec[i] > grey_threshold {
                    vec[i] = BGRA_INVISIBLE_PIXEL.0;
                    vec[i+1] = BGRA_INVISIBLE_PIXEL.1;
                    vec[i+2] = BGRA_INVISIBLE_PIXEL.2;
                    vec[i+3] = BGRA_INVISIBLE_PIXEL.3;
                }
                // transforms shades of grey into fully opaque black, except for white
                else {
                    vec[i] = 0;
                    vec[i+1] = 0;
                    vec[i+2] = 0;
                    vec[i+3] = 255;
                }
            }
            i +=4;
        }
    }
    /// Whites become transparent, range from white to lowest BGR will get a proportionate Alpha value. Where Alpha < 255 no changes will be made (the values of colors with transparency won't be alterated)
    pub fn white_background_to_transparency_gradient(vec :&Vec<u8>) -> Vec<u8> {
        let mut vec_adjusted :Vec<u8> = Vec::with_capacity(vec.len());
    
        let mut j = 0;
        // get Alpha Rred Green Blue values
        let mut lowest_blue = 255;
        let mut lowest_green = 255;
        let mut lowest_red = 255;
        for _ in 0..vec.len()/4 {
            if vec[j] < lowest_blue {
                lowest_blue = vec[j];
            }
            if vec[j+1] < lowest_green {
                lowest_green = vec[j+1];
            }
            if vec[j+2] < lowest_red {
                lowest_red = vec[j+2];
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
        for _ in 0..vec.len()/4 {
            // get Alpha Rred Green Blue values
            let blue = vec[i];
            let green = vec[i+1];
            let red = vec[i+2];
            let alpha = vec[i+3];
            // if pixel has full opacity (no transparency, Alpha is at it's max : 255 (u8))
            if alpha == 255 {
                // and pixel is not white (B=G=R == 255)
                if blue < 255 || green < 255 || red < 255 {
                    // we need to adjust the values of the RGB bytes
    
                    // get the byte we need as adjusting base (this color's byte which in the whole image reached the lowest value (B or G or R))
                    let this_val = vec[i + lowest_val_index];
                    let alpha_adjusted = (255 - this_val + lowest_val) as u8;
                    vec_adjusted.extend_from_slice(&[lowest_blue, lowest_green, lowest_red, alpha_adjusted]);
                }
                // pixel is white, so make it transparent. add lowest BGR values to keep the bytes all with the same BGR values, and only change the Alpha
                else {
                    vec_adjusted.extend_from_slice(&[lowest_blue, lowest_green, lowest_red, 0]);
                }
            }
            // pixel has some transparency (Alpha < 255), leave it as is
            else {
                vec_adjusted.extend_from_slice(&[blue, green, red, alpha]);
            }
    
            i += 4;
        }
        return vec_adjusted;
    }

    /// Returns a new PixelsCollection from the provided one, scaling based on the provided ResizeSize (either width or hight)
    pub fn create_new_resized(&self, resize_size: ResizeSize) -> PixelsCollection<u8> {

        let new_height;
        let new_width;

        match resize_size {
            ResizeSize::Width(w) => {
                new_width = w;
                new_height = (new_width * self.height) / self.width;
            },
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
                let pixel_needed = self.row_length * (fy as f32 * y_ratio) as usize + 4 * (fx as f32 * x_ratio) as usize;
                bytes_resized.push(self.bytes[pixel_needed]);
                bytes_resized.push(self.bytes[pixel_needed+1]);
                bytes_resized.push(self.bytes[pixel_needed+2]);
                bytes_resized.push(self.bytes[pixel_needed+3]);
            }
        }

        return PixelsCollection::create(new_width, new_height, bytes_resized).unwrap();
    }
}

/// Measure of either Width or Height
pub enum ResizeSize {
    Width(usize),
    Height(usize)
}

impl PixelsCollection<u32> {
    pub fn switch_bytes(&mut self, i1 :usize, i2 :usize) {
        <u32>::switch_bytes(&mut self.bytes, i1, i2);
    }
}
