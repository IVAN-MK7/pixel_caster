use crate::{pixels_string::BGRA, PixelValues};

pub trait ColorAlteration<T: PixelValues<T>> {
    /// Set BGR to the provided values
    fn set_bgr(&mut self, b: u8, g: u8, r: u8);
    /// Sets the provided BGRA values to the bytes retrieved by the provided bytes addresses.
    fn set_addresses_bgra(&mut self, bytes_addresses: &[usize], b: u8, g: u8, r: u8, a: u8);
    /// Sets the provided optional BGRA values to the bytes retrieved by the provided bytes addresses.
    /// In case an optional value in not provided (is None) the value of the byte will not be overwritten, will be left as is.
    /// Same as set_addresses_bgra but permits to not override a specific BGRA color, e.g.: overwrite only BG and not RA.
    fn set_addresses_optional_bgra(
        &mut self,
        bytes_addresses: &[usize],
        b: Option<u8>,
        g: Option<u8>,
        r: Option<u8>,
        a: Option<u8>,
    );
    /// Sets the provided optional BGRA values to the bytes retrieved by the provided positions Vec<(x: usize, y: usize)>.
    fn set_positions_bgra(
        &mut self,
        height: usize,
        vec_pos: &[(usize, usize)],
        b: u8,
        g: u8,
        r: u8,
        a: u8,
    );

    /// When a BGR combination is met, it's Alpha will be set to a provided value
    fn match_bgr_set_alpha(&mut self, b: u8, g: u8, r: u8, a: u8);
    /// If an Alpha is met, it's BGR wil be set to the provided BGR
    fn match_alpha_set_bgr(&mut self, a: u8, b: u8, g: u8, r: u8);
    /// If a BGR combination is met, it's BGRA will be set to the provided values
    fn match_bgra_set_bgra(
        &mut self,
        b: u8,
        g: u8,
        r: u8,
        a: u8,
        new_b: u8,
        new_g: u8,
        new_r: u8,
        new_a: u8,
    );

    /// If a color's BGRA values match a statement, set its BGRA to the provided values.
    /// bytes_matcher : |_:u8,_:u8,_:u8,a:u8| -> bool { a > 0}
    fn color_matcher_and_new_color<F: Fn(u8, u8, u8, u8) -> bool>(
        &mut self,
        bytes_matcher: F,
        v0: u8,
        v1: u8,
        v2: u8,
        v3: u8,
    );

    /// If a color's BGRA values match a statement, set its BGRA to the provided values.
    /// bytes_matcher : |_:u8,_:u8,_:u8,a:u8| -> bool { a > 0}
    fn color_matcher_and_alterator<FM: Fn(u8, u8, u8, u8) -> bool, FA: Fn(&mut [u8])>(
        &mut self,
        bytes_matcher: FM,
        bytes_alterator: FA,
    );

    /// if a Alpha is not 255, it's RGB will be set to 0, alpha_pos_in_byte must be between 0 and 3
    fn alpha_not_max_clear_color(&mut self, alpha_pos_in_byte: usize);
}
impl ColorAlteration<u8> for Vec<u8> {
    fn set_bgr(&mut self, b: u8, g: u8, r: u8) {
        let mut i = 0;
        for _ in 0..(self.len() / 4) {
            self[i] = b;
            self[i + 1] = g;
            self[i + 2] = r;
            i += 4;
        }
    }

    fn set_addresses_bgra(&mut self, bytes_addresses: &[usize], b: u8, g: u8, r: u8, a: u8) {
        for byte in bytes_addresses {
            self[*byte] = b;
            self[byte + 1] = g;
            self[byte + 2] = r;
            self[byte + 3] = a;
        }
    }

    fn set_addresses_optional_bgra(
        &mut self,
        bytes_addresses: &[usize],
        b: Option<u8>,
        g: Option<u8>,
        r: Option<u8>,
        a: Option<u8>,
    ) {
        for byte in bytes_addresses {
            if let Some(x) = b {
                self[*byte] = x;
            }
            if let Some(x) = g {
                self[byte + 1] = x;
            }
            if let Some(x) = r {
                self[byte + 2] = x;
            }
            if let Some(x) = a {
                self[byte + 3] = x;
            }
        }
    }

    fn set_positions_bgra(
        &mut self,
        height: usize,
        vec_pos: &[(usize, usize)],
        b: u8,
        g: u8,
        r: u8,
        a: u8,
    ) {
        let stride = self.len() / height;
        for (x, y) in vec_pos {
            let pos = stride * y + 4 * x;
            self[pos] = b;
            self[pos + 1] = g;
            self[pos + 2] = r;
            self[pos + 3] = a;
        }
    }

    fn match_bgr_set_alpha(&mut self, b: u8, g: u8, r: u8, a: u8) {
        let mut i = 0;
        for _ in 0..(self.len() / 4) {
            if self[i] == b && self[i + 1] == g && self[i + 2] == r {
                self[i + 3] = a;
            }
            i += 4;
        }
    }

    fn match_alpha_set_bgr(&mut self, a: u8, b: u8, g: u8, r: u8) {
        let mut i = 0;
        for _ in 0..(self.len() / 4) {
            if self[i + 3] == a {
                self[i] = b;
                self[i + 1] = g;
                self[i + 2] = r;
            }
            i += 4;
        }
    }

    fn match_bgra_set_bgra(
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
        let mut i = 0;
        for _ in 0..(self.len() / 4) {
            if self[i] == b && self[i + 1] == g && self[i + 2] == r && self[i + 3] == a {
                self[i] = new_b;
                self[i + 1] = new_g;
                self[i + 2] = new_r;
                self[i + 3] = new_a;
            }
            i += 4;
        }
    }

    /// If the BGRA values pass the `bytes_matcher` their values will be set to the provided ones
    /// # Examples
    ///
    /// ```no_run
    /// let mut vec = vec![255,255,255,255, 100,0,200,255, 0,255,0,100];
    /// // In the vec there are values for the colors of 3 pixels (each has its BGRA combinations of 4 values) those which have an Alpha of 255
    /// // will have their Blue and Alpha values set to 255, the Green and Red values set to 0, in order to change their color to fully opaque Blue (255,0,0,255).
    /// // In this case the ones that will be changed are the first and the second BGRA combinations of the vec
    /// vec.color_matcher_and_new_color(|_:u8,_:u8,_:u8,a:u8| -> bool {a == 255}, 255,0,0,255);
    /// // this method permits to pass functions as parameters, simplifying the coding process and incrementing the readability
    /// fn fully_opaque(_:u8,_:u8,_:u8,a:u8) -> bool { a == 255 }
    /// vec.color_matcher_and_new_color(fully_opaque, 255,0,0,255);
    /// ```
    fn color_matcher_and_new_color<F: Fn(u8, u8, u8, u8) -> bool>(
        &mut self,
        bytes_matcher: F,
        v0: u8,
        v1: u8,
        v2: u8,
        v3: u8,
    ) {
        let mut i = 0;
        for _ in 0..self.len() / 4 {
            if bytes_matcher(self[i], self[i + 1], self[i + 2], self[i + 3]) {
                self[i] = v0;
                self[i + 1] = v1;
                self[i + 2] = v2;
                self[i + 3] = v3;
            }
            i += 4;
        }
    }

    /// If the BGRA values pass the `bytes_matcher` their values will be alterated by the `bytes_alterator`
    /// # Examples
    ///
    /// ```no_run
    /// let mut vec = vec![255,255,255,255, 100,0,200,255, 0,255,0,100];
    /// // In the vec there are values for the colors of 3 pixels (each has its BGRA combinations of 4 values) those which have a BGRA combination that corresponds to fully opaque white (B,G,R,A = 255)
    /// // will have their Blue and Green values set to 0, in order to change their color to fully opaque Red (0,0,255,255). In this case the one that will be changed is the first BGRA combination of the vec
    /// vec.color_matcher_and_alterator(|b: u8, g: u8, r: u8, a: u8| -> bool { b == 255 && g == 255 && r == 255 && a == 255 }, |bgra: &mut[u8]| { bgra[0] = 0; bgra[1] = 0; });
    /// // this method permits to pass functions as parameters, simplifying the coding process and incrementing the readability
    /// fn visible_not_white(b:u8,g:u8,r:u8,a:u8) -> bool { (b < 255 || g < 255 || r < 255) && a > 0 }
    /// fn all_to_zero(bgra: &mut [u8]) { bgra.fill(0) }
    /// vec.color_matcher_and_alterator(visible_not_white, all_to_zero);
    /// ```
    fn color_matcher_and_alterator<FM: Fn(u8, u8, u8, u8) -> bool, FA: Fn(&mut [u8])>(
        &mut self,
        bytes_matcher: FM,
        bytes_alterator: FA,
    ) {
        let mut i = 0;
        for _ in 0..self.len() / 4 {
            if bytes_matcher(self[i], self[i + 1], self[i + 2], self[i + 3]) {
                bytes_alterator(&mut self[i..=i + 3])
            }
            i += 4;
        }
    }

    fn alpha_not_max_clear_color(&mut self, alpha_pos_in_byte: usize) {
        if alpha_pos_in_byte > 3 {
            return;
        }
        let mut i = 0;
        for _ in 0..(self.len() / 4) {
            if self[i + alpha_pos_in_byte] < 255 {
                self[i] = 0;
                self[i + 1] = 0;
                self[i + 2] = 0;
                self[i + 3] = 0;
            }
            i += 4;
        }
    }
}

pub mod bytes_matchers {
    pub fn visible(_: u8, _: u8, _: u8, a: u8) -> bool {
        a > 0
    }
    pub fn visible_not_white(b: u8, g: u8, r: u8, a: u8) -> bool {
        (b < 255 || g < 255 || r < 255) && a > 0
    }
    pub fn fully_opaque(_: u8, _: u8, _: u8, a: u8) -> bool {
        a == 255
    }
    pub fn not_fully_opaque(_: u8, _: u8, _: u8, a: u8) -> bool {
        a < 255
    }
}
pub mod bytes_alterators {
    /// Sets the values of the given slice to 0
    pub fn all_to_zero(bgra: &mut [u8]) {
        bgra.fill(0)
    }
    pub fn transparent_white(bgra: &mut [u8]) {
        bgra[0] = 255;
        bgra[1] = 255;
        bgra[2] = 255;
        bgra[3] = 0;
        //slice.chunks_mut(4).for_each(|chunk| { chunk[0] = 0; chunk[1] = 0; chunk[2] = 0; chunk[3] = 0; })
    }
}

/// Returns lowest B,G,R and highest A found in colors where their A >= 1
pub fn image_lowest_visible_bgr(vec: &[u8]) -> BGRA<u8> {
    let mut j = 0;
    // get Alpha Rred Green Blue values
    let mut lowest_blue = 255;
    let mut lowest_green = 255;
    let mut lowest_red = 255;
    let mut highest_alpha = 1;
    for _ in 0..vec.len() / 4 {
        if vec[j + 3] >= highest_alpha {
            highest_alpha = vec[j + 3];
            if vec[j] < lowest_blue {
                lowest_blue = vec[j];
            }
            if vec[j + 1] < lowest_green {
                lowest_green = vec[j + 1];
            }
            if vec[j + 2] < lowest_red {
                lowest_red = vec[j + 2];
            }
        }
        j += 4;
    }
    BGRA(lowest_blue, lowest_green, lowest_red, highest_alpha)
}

/// Returns B,G,R of the last found color with the heighest A
pub fn image_opaquest_bgr(vec: &[u8]) -> Vec<u8> {
    let mut j = 0;
    // get Alpha Red Green Blue values
    let mut blue = 255;
    let mut green = 255;
    let mut red = 255;
    let mut highest_alpha = 0;
    for _ in 0..vec.len() / 4 {
        if vec[j + 3] >= highest_alpha {
            highest_alpha = vec[j + 3];
            blue = vec[j];
            green = vec[j + 1];
            red = vec[j + 2];
        }
        j += 4;
    }
    vec![blue, green, red, highest_alpha]
}

/// Switches pixel's color's BGRA bytes positions.
pub trait SwitchBytes<T: crate::PixelValues<T>, U: crate::PixelValues<U>> {
    fn switch_bytes(vec: &mut Vec<T>, v1: usize, v2: usize);
    fn swap_blue_with_red(vec: &[T]) -> Vec<T>;
    fn u8_u32_casting(vec: &[T]) -> Vec<U>;
}
impl SwitchBytes<u8, u32> for u8 {
    /// Switches values of 2 provided indexes of every 4 8-bytes chunks ((u8 \[B,G,R,A\]) B: u8, G: u8, R:u8, A :u8 values chunks) in the vector, i1 and i2 must be between 0 and 3 (B:0, G:1, R:2, A:3).
    fn switch_bytes(vec: &mut Vec<u8>, i1: usize, i2: usize) {
        if i1 > 3 || i2 > 3 || i1 == i2 || vec.len() % 4 != 0 {
            return;
        }
        let mut i = 0;
        for _ in 0..vec.len() / 4 {
            vec.swap(i + i1, i + i2);
            i += 4;
        }
    }
    /// Returns a cloned Vec<u8> with RGBA values sequence, if the provided Vec<u8> values were already in RGBA will be returned in BGRA
    fn swap_blue_with_red(vec_bgra: &[u8]) -> Vec<u8> {
        let mut vec_rgba = vec_bgra.to_vec();
        <u8>::switch_bytes(&mut vec_rgba, 0, 2);
        vec_rgba
    }
    fn u8_u32_casting(vec: &[u8]) -> Vec<u32> {
        let mut vec_u32 = Vec::with_capacity(vec.len() / 4);
        let mut i = 0;
        for _ in 0..vec.len() / 4 {
            // native endian is little endian in my case (Intel CPU)
            vec_u32.extend([u32::from_ne_bytes([
                vec[i],
                vec[i + 1],
                vec[i + 2],
                vec[i + 3],
            ])]);
            i += 4;
        }
        vec_u32
    }
}
impl SwitchBytes<u32, u8> for u32 {
    /// Switches values of 2 provided indexes of every 4 8-bytes chunks (B: u8, G: u8, R: u8, A: u8 values chunks) in the vector, i1 and i2 must be between 0 and 3 (B:0, G:1, R:2, A:3).
    fn switch_bytes(vec: &mut Vec<u32>, i1: usize, i2: usize) {
        if i1 > 3 || i2 > 3 || i1 == i2 {
            return;
        }

        // u8 [B,G,R,A] -> u32 on little endian CPU the order reversed, becomes : [0xARGB]
        // If CPU endianness is little then when 4 u8 bytes (B = 100 (0x64), G = 150 (0x96), R = 200 (0xC8), A = 255 (0xFF))
        // will be ported into a u32 value, the values will be ARGB, because the values will start on the right of the u32 value, so 0xFFC8_9664
        let mut byte_to_switch_1_index = i1 * 8;
        let mut byte_to_switch_2_index = i2 * 8;
        let mut byte_to_switch_1_full_val: u32 = 0xFF << (i1 * 8);
        let mut byte_to_switch_2_full_val: u32 = 0xFF << (i2 * 8);
        if cfg!(target_endian = "big") {
            byte_to_switch_1_index = 24 - (i1 * 8);
            byte_to_switch_2_index = 24 - (i1 * 8);
            byte_to_switch_1_full_val = 0xFF << (24 - (i1 * 8));
            byte_to_switch_2_full_val = 0xFF << (24 - (i2 * 8));
        }

        // gets each of the 4 values index of the position from which the values start, starting from right
        // and also the u32 value of the value's bytes on full value, where the other 3 values bytes are set to value 0
        let (ordered_indexes, fullvalues) = u32_bytes_oredered_indexes_and_fullvalues();
        let v1_index = ordered_indexes[0];
        let v2_index = ordered_indexes[1];
        let v3_index = ordered_indexes[2];
        let v4_index = ordered_indexes[3];
        let v1_full_val = fullvalues[0];
        let v2_full_val = fullvalues[1];
        let v3_full_val = fullvalues[2];
        let v4_full_val = fullvalues[3];

        let mut v1_shift = v1_index;
        let mut v2_shift = v2_index;
        let mut v3_shift = v3_index;
        let mut v4_shift = v4_index;

        // & (and) : the resulting value contains bits where both had them
        // | (or) : the resulting value contains bits where any of them had them
        // if B(GRA) is either the first or the second byte of those to be switched
        // ( in case B(GRA)'s byte's full value matches with either i1 or i2's byte's full value : when it's value will be shifted to the right for enough positions (v1_index)
        // the resulting byte's full value will be 255(u8)/0xFF(HEX) )
        // check if v1 position is to be switched :
        // let is_v1_to_switch: bool = ( (v1_full_val & byte_to_switch_1_full_val) | (v1_full_val & byte_to_switch_2_full_val) ) >> v1_index == 0xFF;
        // example on little endian CPU with byte_to_switch_1_full_val = 0x0000_00FF and byte_to_switch_2_full_val = 0x00FF_0000 (so in a BGRA we want to switch B with R, and with little endian the order in u32 is turned into 0xARGB) :
        // let is_v1_to_switch: bool = ( (0x0000_00FF & 0x0000_00FF) | (0x0000_00FF & 0x00FF_0000) ) >> 0 == 0xFF; -> true, so 0x0000_00FF is to be switched (0x0000_00BB Blue is to be switched position with the other color)
        // let is_v2_to_switch: bool = ( (0x0000_FF00 & 0x0000_00FF) | (0x0000_FF00 & 0x00FF_0000) ) >> 0 == 0xFF; -> false, so 0x0000_FF00 is NOT to be switched (0x0000_GG00 Green stays in the same position)
        // let is_v3_to_switch: bool = ( (0x00FF_0000 & 0x0000_00FF) | (0x00FF_0000 & 0x00FF_0000) ) >> 0 == 0xFF; -> true, so 0x0000_00FF is to be switched (0x00RR_0000 so Red is to be switched position with the other color, Blue)
        // let is_v4_to_switch: bool = ( (0xFF00_0000 & 0x0000_00FF) | (0xFF00_0000 & 0x00FF_0000) ) >> 0 == 0xFF; -> false, so 0x0000_00FF is NOT to be switched (0xAA00_0000 Alpha stays in the same position)
        // the order will be therefore changed from 0xARGB into 0xABGR.
        // If the CPU endian was big it would have change from 0xBGRA into 0xRGBA, because u8 [B,G,R,A] -> u32 on big endian CPU the order becomes becomes : [0xBGRA], after the switch : [0xRGBA]

        if (v1_full_val & byte_to_switch_1_full_val) >> v1_index == 0xFF {
            v1_shift = byte_to_switch_2_index;
        } else if (v1_full_val & byte_to_switch_2_full_val) >> v1_index == 0xFF {
            v1_shift = byte_to_switch_1_index;
        }
        if (v2_full_val & byte_to_switch_1_full_val) >> v2_index == 0xFF {
            v2_shift = byte_to_switch_2_index;
        } else if (v2_full_val & byte_to_switch_2_full_val) >> v2_index == 0xFF {
            v2_shift = byte_to_switch_1_index;
        }
        if (v3_full_val & byte_to_switch_1_full_val) >> v3_index == 0xFF {
            v3_shift = byte_to_switch_2_index;
        } else if (v3_full_val & byte_to_switch_2_full_val) >> v3_index == 0xFF {
            v3_shift = byte_to_switch_1_index;
        }
        if (v4_full_val & byte_to_switch_1_full_val) >> v4_index == 0xFF {
            v4_shift = byte_to_switch_2_index;
        } else if (v4_full_val & byte_to_switch_2_full_val) >> v4_index == 0xFF {
            v4_shift = byte_to_switch_1_index;
        }

        let mut v1: u32;
        let mut v2: u32;
        let mut v3: u32;
        let mut v4: u32;
        for p in vec {
            // get Blue Green Red Alpha values by excluding the others
            // for each make null the bytes that are not those representing the value we need
            v1 = (*p & v1_full_val) >> v1_index; // get pixel bytes in the wanted order (probably, e.g.: BGRA, but doesn't matter if it's another)
            v2 = (*p & v2_full_val) >> v2_index;
            v3 = (*p & v3_full_val) >> v3_index;
            v4 = (*p & v4_full_val) >> v4_index;
            *p = (v1 << v1_shift) | (v2 << v2_shift) | (v3 << v3_shift) | (v4 << v4_shift);
        }
    }
    /// Returns a cloned Vec<u8> with RGBA values sequence, if the provided Vec<u8> values were already in RGBA will be returned in BGRA
    fn swap_blue_with_red(vec_bgra: &[u32]) -> Vec<u32> {
        let mut vec_rgba = vec_bgra.to_vec();
        <u32>::switch_bytes(&mut vec_rgba, 0, 2);
        vec_rgba
    }
    fn u8_u32_casting(vec: &[u32]) -> Vec<u8> {
        let mut vec_u32 = Vec::with_capacity(vec.len() * 4);
        for val in vec {
            vec_u32.extend(val.to_ne_bytes());
        }
        vec_u32
    }
}

/// Returns the indexes of the positions and the full values of the 4 u8 values in a u32 value given current CPU's endianness.
///
/// In a Vec<u8> every 4 pixels will represent a value, e.g: from left to right: B, G, R, A.
/// When turning the Vec<u8> into a Vec<u32> every 4 bytes will become one (8 x 4 = 32),
/// when doing so on a CPU with little endianness the 4 values will have their order reversed, so from left to right: A, R, G, B
///
/// u8 [B,G,R,A] -> u32 on little endian CPU the order is reversed, becomes : u32 [0xARGB]
/// If CPU endianness is little then when 4 u8 bytes (B = 100 (0x64), G = 150 (0x96), R = 200 (0xC8), A = 255 (0xFF))
/// will be ported into a u32 value, the values will be on a ARGB order, because the values will go from right to the left of the u32 value, so 0xFFC8_9664
///
pub fn u32_bytes_oredered_indexes_and_fullvalues() -> ([usize; 4], [u32; 4]) {
    let mut v1_index = 0;
    let mut v2_index = 8;
    let mut v3_index = 16;
    let mut v4_index = 24;
    let mut v1_full_val = 0x0000_00FF;
    let mut v2_full_val = 0x0000_FF00;
    let mut v3_full_val = 0x00FF_0000;
    let mut v4_full_val = 0xFF00_0000;
    if cfg!(target_endian = "big") {
        v1_index = 24;
        v2_index = 16;
        v3_index = 8;
        v4_index = 0;
        v1_full_val = 0xFF00_0000;
        v2_full_val = 0x00FF_0000;
        v3_full_val = 0x0000_FF00;
        v4_full_val = 0x0000_00FF;
    }
    (
        [v1_index, v2_index, v3_index, v4_index],
        [v1_full_val, v2_full_val, v3_full_val, v4_full_val],
    )
}

#[cfg(test)]
mod tests {
    use crate::{bgra_management::*, PixelsCollection, PixelsSendMode, Screen};

    #[test]
    fn test_u8_u32_convertion() {
        let mut bytes_u8_bgra: Vec<u8> = Vec::new();
        //                                B    G  R   A
        //                                blue
        bytes_u8_bgra.extend_from_slice(&[255, 0, 0, 125]);
        //                                  green
        bytes_u8_bgra.extend_from_slice(&[0, 255, 0, 125]);
        //                                      red
        bytes_u8_bgra.extend_from_slice(&[0, 0, 255, 125]);

        Screen::update_area_custom(
            &bytes_u8_bgra,
            0,
            0,
            bytes_u8_bgra.len() as u32 / 4,
            1,
            PixelsSendMode::AlphaEnabled,
        );
        // when exporting into .ong we need to go from BGRA to RGBA, so swap the B and R values
        let mut bytes_u8_rgba_from_u8_bgra = bytes_u8_bgra.clone();
        <u8>::switch_bytes(&mut bytes_u8_rgba_from_u8_bgra, 0, 2);
        image::save_buffer_with_format(
            format!("{}{}", "media/", "bytes_u8_rgba_from_u8_bgra_export.png"),
            &bytes_u8_rgba_from_u8_bgra,
            (bytes_u8_rgba_from_u8_bgra.len() / 4).try_into().unwrap(),
            1,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();

        // u8 [B,G,R,A] -> u32 on little endian CPU the order becomes becomes : [0xARGB]
        let bytes_u32_bgra = <u8>::u8_u32_casting(&bytes_u8_bgra);
        // A = 125, R = 0, G = 0, B = 255
        assert_eq!(bytes_u32_bgra[0], 0x7D00_00FF);
        // on big endian CPU we would have compared it to 0xFF00_007D, because the order would be [0xBGRA]
        Screen::update_area_custom(
            &bytes_u32_bgra,
            0,
            0,
            bytes_u32_bgra.len() as u32,
            1,
            PixelsSendMode::AlphaEnabled,
        );

        let mut bytes_u32_rgba_from_u32_bgra = bytes_u32_bgra.clone();
        <u32>::switch_bytes(&mut bytes_u32_rgba_from_u32_bgra, 0, 2);
        // A = 125, R = 255, G = 0, B = 0 , because we switched the value of B (index 0 in BGRA) with R (index 2 in BGRA)
        assert_eq!(bytes_u32_rgba_from_u32_bgra[0], 0x7DFF_0000);
        // on big endian CPU we would have compared it to 0x0000_FF7D, because the order would be [0xBGRA]

        let bytes_u8_rgba_from_u32_rgba = <u32>::u8_u32_casting(&bytes_u32_rgba_from_u32_bgra);
        // when exporting into .ong we need RGBA values
        image::save_buffer_with_format(
            format!("{}{}", "media/", "bytes_u8_rgba_from_u32_rgba_export.png"),
            &bytes_u8_rgba_from_u32_rgba,
            (bytes_u8_rgba_from_u32_rgba.len() / 4).try_into().unwrap(),
            1,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }

    #[test]
    fn test_u8_u32_convertion_png_with_transparency() {
        let image_u8_bgra = PixelsCollection::<u8>::from_png(
            "media/Logo_MK7_Transparent_Bg_ColorsWithHalfAlpha.png",
        )
        .unwrap();

        let image_u8_bgra_from_image_rgba = image_u8_bgra.clone();
        Screen::update_area_custom(
            &image_u8_bgra_from_image_rgba.bytes,
            -200,
            0,
            image_u8_bgra.width as u32,
            image_u8_bgra.height as u32,
            PixelsSendMode::AlphaEnabled,
        );
        // when exporting into .png we need to go from BGRA to RGBA, so swap the B and R values
        let mut image_u8_rgba_from_image_u8_bgra = image_u8_bgra_from_image_rgba.clone();
        image_u8_rgba_from_image_u8_bgra.switch_bytes(0, 2);
        image::save_buffer_with_format(
            format!("{}{}", "media/", "rgba_u8_export.png"),
            &image_u8_rgba_from_image_u8_bgra.bytes,
            image_u8_bgra.width.try_into().unwrap(),
            image_u8_bgra.height.try_into().unwrap(),
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();

        let rgba_u32 = <u8>::u8_u32_casting(&image_u8_bgra.bytes);
        let image_u32_rgba =
            PixelsCollection::<u32>::create(image_u8_bgra.width, image_u8_bgra.height, rgba_u32)
                .unwrap();
        Screen::update_area_custom(
            &image_u32_rgba.bytes,
            -200,
            0,
            image_u8_bgra.width as u32,
            image_u8_bgra.height as u32,
            PixelsSendMode::AlphaEnabled,
        );

        // image_u32_rgba has BGRA ordered bytes
        let mut bytes_u8_bgra_from_u32_bgra = <u32>::u8_u32_casting(&image_u32_rgba.bytes);
        Screen::update_area_custom(
            &bytes_u8_bgra_from_u32_bgra,
            -200,
            0,
            image_u8_bgra.width as u32,
            image_u8_bgra.height as u32,
            PixelsSendMode::AlphaEnabled,
        );
        // when exporting into .png we need to go from BGRA to RGBA, so swap the B and R values
        <u8>::switch_bytes(&mut bytes_u8_bgra_from_u32_bgra, 0, 2);
        image::save_buffer_with_format(
            format!("{}{}", "media/", "bytes_u8_rgba_from_u32_rgba_export.png"),
            &bytes_u8_bgra_from_u32_bgra,
            image_u8_bgra.width.try_into().unwrap(),
            image_u8_bgra.height.try_into().unwrap(),
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }
}
