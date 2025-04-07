use rgb::RGB8;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::error::{ColorMixerError, Result};

/// Maximum number of colors that can be mixed
const MAX_COLORS: usize = 1000;

/// Color representation that stores RGB values
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    rgb: RGB8,
}

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            rgb: RGB8::new(r, g, b),
        }
    }

    /// Get the RGB components of the color
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.rgb.r, self.rgb.g, self.rgb.b)
    }

    /// Get the hex representation of the color
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.rgb.r, self.rgb.g, self.rgb.b)
    }

    /// Compare color to standard yellow
    pub fn is_yellow(&self) -> bool {
        self.rgb.r == 255 && self.rgb.g == 237 && self.rgb.b == 0
    }

    /// Compare color to standard blue
    pub fn is_blue(&self) -> bool {
        self.rgb.r == 0 && self.rgb.g == 71 && self.rgb.b == 171
    }

    /// Compare color to light yellow
    pub fn is_light_yellow(&self) -> bool {
        self.rgb.r == 255 && self.rgb.g == 249 && self.rgb.b == 128
    }

    /// Compare color to dark yellow
    pub fn is_dark_yellow(&self) -> bool {
        self.rgb.r == 204 && self.rgb.g == 187 && self.rgb.b == 0
    }

    /// Compare color to light blue
    pub fn is_light_blue(&self) -> bool {
        self.rgb.r == 102 && self.rgb.g == 153 && self.rgb.b == 255
    }

    /// Compare color to dark blue
    pub fn is_dark_blue(&self) -> bool {
        self.rgb.r == 0 && self.rgb.g == 32 && self.rgb.b == 91
    }
}

impl FromStr for Color {
    type Err = ColorMixerError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            // Yellow shades
            "yellow" | "#ffed00" => Ok(Self::new(255, 237, 0)),
            "light-yellow" | "#fff980" => Ok(Self::new(255, 249, 128)),
            "dark-yellow" | "#ccbb00" => Ok(Self::new(204, 187, 0)),

            // Blue shades
            "blue" | "#0047ab" => Ok(Self::new(0, 71, 171)),
            "light-blue" | "#6699ff" => Ok(Self::new(102, 153, 255)),
            "dark-blue" | "#00205b" => Ok(Self::new(0, 32, 91)),

            _ => Err(ColorMixerError::UnsupportedColor(
                format!("Unsupported color: {}. Please use one of the predefined yellow or blue shades.", s)
            )),
        }
    }
}

/// Request for adding a color to the mixer
#[derive(Debug, Deserialize)]
pub struct AddColorRequest {
    /// The color to add ("yellow" or "blue")
    pub color: String,
    /// The shade of the color ("light", "standard", or "dark")
    #[serde(default = "default_shade")]
    pub shade: String,
    /// The quantity of the color to add (default: 1)
    #[serde(default = "default_quantity")]
    pub quantity: u32,
}

/// Default quantity for color addition
fn default_quantity() -> u32 {
    1
}

/// Default shade for color addition
fn default_shade() -> String {
    "standard".to_string()
}

/// Color mixer with RGB support
#[derive(Debug, Clone)]
pub struct ColorMixer {
    /// List of colors currently in the mixer
    colors: Vec<Color>,
    /// Maximum number of colors allowed
    max_colors: usize,
}

impl ColorMixer {
    /// Create a new color mixer
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            max_colors: MAX_COLORS,
        }
    }

    /// Add multiple units of a color at once
    pub fn add_colors_str(&mut self, color_str: &str, shade: &str, quantity: u32) -> Result<()> {
        // Construct the full color name with shade
        let full_color_name = if shade == "standard" {
            color_str.to_string()
        } else {
            format!("{}-{}", shade, color_str)
        };

        // Validate the color first to avoid partial additions if the color is invalid
        let color = Color::from_str(&full_color_name)?;

        // Check if we have enough space for all colors
        let current_count = self.colors.len();
        let quantity_usize = quantity as usize;

        if current_count + quantity_usize > self.max_colors {
            return Err(ColorMixerError::MaxColorsReached);
        }

        // Add the colors
        for _ in 0..quantity {
            self.colors.push(color.clone());
        }

        Ok(())
    }

    /// Clear all colors from the mixer
    pub fn clear(&mut self) {
        self.colors.clear();
    }

    /// Get the currently mixed color
    pub fn get_mixed_color(&self) -> Result<Color> {
        if self.colors.is_empty() {
            return Err(ColorMixerError::NoColors);
        }

        // If only one color, return it directly
        if self.colors.len() == 1 {
            return Ok(self.colors[0].clone());
        }

        // Count the number of each color shade
        let mut std_yellow_count = 0;
        let mut light_yellow_count = 0;
        let mut dark_yellow_count = 0;
        let mut std_blue_count = 0;
        let mut light_blue_count = 0;
        let mut dark_blue_count = 0;

        for color in &self.colors {
            if color.is_yellow() {
                std_yellow_count += 1;
            } else if color.is_light_yellow() {
                light_yellow_count += 1;
            } else if color.is_dark_yellow() {
                dark_yellow_count += 1;
            } else if color.is_blue() {
                std_blue_count += 1;
            } else if color.is_light_blue() {
                light_blue_count += 1;
            } else if color.is_dark_blue() {
                dark_blue_count += 1;
            }
        }

        // Calculate total counts for each color family
        let yellow_count = std_yellow_count + light_yellow_count + dark_yellow_count;
        let blue_count = std_blue_count + light_blue_count + dark_blue_count;
        let total = yellow_count + blue_count;

        // If there's only one color family, calculate the average of that family's shades
        if yellow_count > 0 && blue_count == 0 {
            // Only yellow shades
            let r_sum = (255 * std_yellow_count) + (255 * light_yellow_count) + (204 * dark_yellow_count);
            let g_sum = (237 * std_yellow_count) + (249 * light_yellow_count) + (187 * dark_yellow_count);
            let b_sum = (0 * std_yellow_count) + (128 * light_yellow_count) + (0 * dark_yellow_count);

            let r = (r_sum / yellow_count) as u8;
            let g = (g_sum / yellow_count) as u8;
            let b = (b_sum / yellow_count) as u8;

            return Ok(Color::new(r, g, b));
        } else if blue_count > 0 && yellow_count == 0 {
            // Only blue shades
            let r_sum = (0 * std_blue_count) + (102 * light_blue_count) + (0 * dark_blue_count);
            let g_sum = (71 * std_blue_count) + (153 * light_blue_count) + (32 * dark_blue_count);
            let b_sum = (171 * std_blue_count) + (255 * light_blue_count) + (91 * dark_blue_count);

            let r = (r_sum / blue_count) as u8;
            let g = (g_sum / blue_count) as u8;
            let b = (b_sum / blue_count) as u8;

            return Ok(Color::new(r, g, b));
        }

        // Mix yellow and blue shades
        let yellow_ratio = yellow_count as f32 / total as f32;
        let blue_ratio = blue_count as f32 / total as f32;

        // Calculate weighted average for each color family
        let yellow_r = if yellow_count > 0 {
            ((255.0 * std_yellow_count as f32) + (255.0 * light_yellow_count as f32) + (204.0 * dark_yellow_count as f32)) / yellow_count as f32
        } else { 0.0 };

        let yellow_g = if yellow_count > 0 {
            ((237.0 * std_yellow_count as f32) + (249.0 * light_yellow_count as f32) + (187.0 * dark_yellow_count as f32)) / yellow_count as f32
        } else { 0.0 };

        let yellow_b = if yellow_count > 0 {
            ((0.0 * std_yellow_count as f32) + (128.0 * light_yellow_count as f32) + (0.0 * dark_yellow_count as f32)) / yellow_count as f32
        } else { 0.0 };

        let blue_r = if blue_count > 0 {
            ((0.0 * std_blue_count as f32) + (102.0 * light_blue_count as f32) + (0.0 * dark_blue_count as f32)) / blue_count as f32
        } else { 0.0 };

        let blue_g = if blue_count > 0 {
            ((71.0 * std_blue_count as f32) + (153.0 * light_blue_count as f32) + (32.0 * dark_blue_count as f32)) / blue_count as f32
        } else { 0.0 };

        let blue_b = if blue_count > 0 {
            ((171.0 * std_blue_count as f32) + (255.0 * light_blue_count as f32) + (91.0 * dark_blue_count as f32)) / blue_count as f32
        } else { 0.0 };

        // Final color mixing
        let r = (yellow_r * yellow_ratio + blue_r * blue_ratio) as u8;
        let g = ((yellow_g * yellow_ratio) + (blue_g * blue_ratio)) as u8;
        let b = ((yellow_b * yellow_ratio) + (blue_b * blue_ratio)) as u8;

        Ok(Color::new(r, g, b))
    }
}
