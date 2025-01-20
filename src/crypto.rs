use js_sys::Uint8Array;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

/// Size of the DCT block (8x8 pixels)
const N: usize = 8;

/// Standard JPEG quantization table for luminance
const QUANTIZATION_TABLE: [u8; 64] = [
    16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57, 69, 56,
    14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64, 81, 104, 113,
    92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95, 98, 112, 100, 103, 99,
];

/// Represents an image that can be processed for steganography
#[wasm_bindgen]
pub struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Image {
    /// Creates a new Image instance
    ///
    /// # Arguments
    ///
    /// * `pixels` - A Uint8Array containing the pixel data
    /// * `width` - The width of the image
    /// * `height` - The height of the image
    ///
    /// # Returns
    ///
    /// A new Image instance
    #[wasm_bindgen(constructor)]
    pub fn new(pixels: &[u8], width: u8, height: u8) -> Image {
        let pixels_vec = pixels.to_vec();
        Image {
            pixels: pixels_vec,
            width: width as usize,
            height: height as usize,
        }
    }

    /// Embeds a message into the image using DCT-based steganography
    ///
    /// # Arguments
    ///
    /// * `message` - The message to embed
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    #[wasm_bindgen]
    pub fn embed_message(&mut self, message: &str) -> Result<(), JsValue> {
        let mut dct_blocks = self
            .to_dct_blocks()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let mut message_bits = message
            .bytes()
            .flat_map(|byte| (0..8).map(move |i| (byte >> i) & 1));

        for block in dct_blocks.iter_mut() {
            if let Some(bit) = message_bits.next() {
                // Modify the DC coefficient based on the bit
                block[0] = if bit == 1 {
                    block[0].ceil()
                } else {
                    block[0].floor()
                };
            } else {
                break;
            }
        }

        self.from_dct_blocks(dct_blocks)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Extracts a hidden message from the image
    ///
    /// # Returns
    ///
    /// A Result containing the extracted message or an error
    #[wasm_bindgen]
    pub fn extract_message(&self) -> Result<String, JsValue> {
        let dct_blocks = self
            .to_dct_blocks()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let mut message = Vec::new();
        let mut byte = 0u8;
        let mut bit_count = 0;

        for block in dct_blocks.iter() {
            let bit = if block[0].fract() >= 0.5 { 1 } else { 0 };
            byte |= bit << bit_count;
            bit_count += 1;

            if bit_count == 8 {
                message.push(byte);
                byte = 0;
                bit_count = 0;
            }
        }

        String::from_utf8(message).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Returns the pixel data of the image
    ///
    /// # Returns
    ///
    /// A Uint8Array containing the pixel data
    #[wasm_bindgen]
    pub fn get_pixels(&self) -> Uint8Array {
        Uint8Array::from(self.pixels.as_slice())
    }
}

/// Trait defining DCT operations
trait Dct {
    fn quantize(&self, dct: Vec<f64>) -> Vec<f64>;
    fn unquantize(&self, quantized: Vec<f64>) -> Vec<f64>;
    fn forward_dct(&self, block: &[u8]) -> Vec<f64>;
    fn inverse_dct(&self, dct: &[f64]) -> Vec<u8>;
}

impl Dct for Image {
    /// Quantizes DCT coefficients
    ///
    /// # Arguments
    ///
    /// * `dct` - A vector of DCT coefficients
    ///
    /// # Returns
    ///
    /// A vector of quantized DCT coefficients
    fn quantize(&self, dct: Vec<f64>) -> Vec<f64> {
        dct.iter()
            .zip(QUANTIZATION_TABLE.iter())
            .map(|(&d, &q)| (d / q as f64).round())
            .collect()
    }

    /// Unquantizes DCT coefficients
    ///
    /// # Arguments
    ///
    /// * `quantized` - A vector of quantized DCT coefficients
    ///
    /// # Returns
    ///
    /// A vector of unquantized DCT coefficients
    fn unquantize(&self, quantized: Vec<f64>) -> Vec<f64> {
        quantized
            .iter()
            .zip(QUANTIZATION_TABLE.iter())
            .map(|(&q, &t)| q * t as f64)
            .collect()
    }

    /// Applies forward DCT to an 8x8 block
    ///
    /// # Arguments
    ///
    /// * `block` - An 8x8 block of pixel values
    ///
    /// # Returns
    ///
    /// A vector of DCT coefficients
    fn forward_dct(&self, block: &[u8]) -> Vec<f64> {
        let mut dct = vec![0.0; N * N];
        for u in 0..N {
            for v in 0..N {
                let mut sum = 0.0;
                for x in 0..N {
                    for y in 0..N {
                        let cos_x = (PI * u as f64 * (2.0 * x as f64 + 1.0)) / (2.0 * N as f64);
                        let cos_y = (PI * v as f64 * (2.0 * y as f64 + 1.0)) / (2.0 * N as f64);
                        sum += block[y * N + x] as f64 * cos_x.cos() * cos_y.cos();
                    }
                }
                let cu = if u == 0 { 1.0 / 2.0_f64.sqrt() } else { 1.0 };
                let cv = if v == 0 { 1.0 / 2.0_f64.sqrt() } else { 1.0 };
                dct[v * N + u] = 0.25 * cu * cv * sum;
            }
        }
        dct
    }

    /// Applies inverse DCT to an 8x8 block of DCT coefficients
    ///
    /// # Arguments
    ///
    /// * `dct` - A vector of DCT coefficients
    ///
    /// # Returns
    ///
    /// A vector of pixel values
    fn inverse_dct(&self, dct: &[f64]) -> Vec<u8> {
        let mut block = vec![0.0; N * N];
        for x in 0..N {
            for y in 0..N {
                let mut sum = 0.0;
                for u in 0..N {
                    for v in 0..N {
                        let cu = if u == 0 { 1.0 / 2.0_f64.sqrt() } else { 1.0 };
                        let cv = if v == 0 { 1.0 / 2.0_f64.sqrt() } else { 1.0 };
                        let cos_x = (PI * u as f64 * (2.0 * x as f64 + 1.0)) / (2.0 * N as f64);
                        let cos_y = (PI * v as f64 * (2.0 * y as f64 + 1.0)) / (2.0 * N as f64);
                        sum += cu * cv * dct[v * N + u] * cos_x.cos() * cos_y.cos();
                    }
                }
                block[y * N + x] = 0.25 * sum;
            }
        }
        block
            .into_iter()
            .map(|p| p.round().clamp(0.0, 255.0) as u8)
            .collect()
    }
}

impl Image {
    /// Converts the image to DCT blocks
    ///
    /// # Returns
    ///
    /// A Result containing a vector of DCT blocks or an error
    fn to_dct_blocks(&self) -> Result<Vec<Vec<f64>>, String> {
        let mut dct_blocks = Vec::new();
        for y in (0..self.height).step_by(N) {
            for x in (0..self.width).step_by(N) {
                let mut block = Vec::with_capacity(N * N);
                for dy in 0..N {
                    for dx in 0..N {
                        let px = x + dx;
                        let py = y + dy;
                        if px < self.width && py < self.height {
                            block.push(self.pixels[py * self.width + px] as f64);
                        } else {
                            block.push(0.0);
                        }
                    }
                }
                let block_u8: Vec<u8> = block.iter().map(|&f| f.round() as u8).collect();
                let dct = self.forward_dct(&block_u8[..]);
                let quantized = self.quantize(dct);
                dct_blocks.push(quantized);
                /***
                    let dct = self.forward_dct(&block);
                    let quantized = self.quantize(dct);
                    dct_blocks.push(quantized);
                ***/
            }
        }
        Ok(dct_blocks)
    }

    /// Converts DCT blocks back to image pixels
    ///
    /// # Arguments
    ///
    /// * `dct_blocks` - A vector of DCT blocks
    ///
    /// # Returns
    ///
    /// A Result indicating success or an error
    fn from_dct_blocks(&mut self, dct_blocks: Vec<Vec<f64>>) -> Result<(), String> {
        let mut block_index = 0;
        for y in (0..self.height).step_by(N) {
            for x in (0..self.width).step_by(N) {
                if block_index >= dct_blocks.len() {
                    return Ok(());
                }
                let unquantized = self.unquantize(dct_blocks[block_index].clone());
                let block = self.inverse_dct(&unquantized);
                for (i, pixel) in block.iter().enumerate() {
                    let px = x + (i % N);
                    let py = y + (i / N);
                    if px < self.width && py < self.height {
                        self.pixels[py * self.width + px] = *pixel;
                    }
                }
                block_index += 1;
            }
        }
        Ok(())
    }
}
