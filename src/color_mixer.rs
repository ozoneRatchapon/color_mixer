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

    /// Compare color to known exact values
    pub fn is_yellow(&self) -> bool {
        self.rgb.r == 255 && self.rgb.g == 237 && self.rgb.b == 0
    }

    /// Compare color to known exact values
    pub fn is_blue(&self) -> bool {
        self.rgb.r == 0 && self.rgb.g == 71 && self.rgb.b == 171
    }
}

impl FromStr for Color {
    type Err = ColorMixerError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "yellow" | "#ffed00" => Ok(Self::new(255, 237, 0)),
            "blue" | "#0047ab" => Ok(Self::new(0, 71, 171)),
            _ => Err(ColorMixerError::UnsupportedColor(
                format!("Only 'yellow' (#FFED00) and 'blue' (#0047AB) are supported. Got: {}", s)
            )),
        }
    }
}

/// Request for adding a color to the mixer
#[derive(Debug, Deserialize)]
pub struct AddColorRequest {
    /// The color to add ("yellow" or "blue")
    pub color: String,
    /// The quantity of the color to add (default: 1)
    #[serde(default = "default_quantity")]
    pub quantity: u32,
}

/// Default quantity for color addition
fn default_quantity() -> u32 {
    1
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
    pub fn add_colors_str(&mut self, color_str: &str, quantity: u32) -> Result<()> {
        // Validate the color first to avoid partial additions if the color is invalid
        let color = Color::from_str(color_str)?;

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

        // Count the number of each color
        let mut yellow_count = 0;
        let mut blue_count = 0;

        for color in &self.colors {
            if color.is_yellow() {
                yellow_count += 1;
            } else if color.is_blue() {
                blue_count += 1;
            }
        }

        // If there's only one type of color, return that exact color
        if yellow_count > 0 && blue_count == 0 {
            return Ok(Color::new(255, 237, 0)); // Return exact yellow (#FFED00)
        } else if blue_count > 0 && yellow_count == 0 {
            return Ok(Color::new(0, 71, 171)); // Return exact blue (#0047AB)
        }

        // Custom mixing formula for yellow and blue
        let total = yellow_count + blue_count;
        let yellow_ratio = yellow_count as f32 / total as f32;
        let blue_ratio = blue_count as f32 / total as f32;

        // Yellow: #FFED00 (255, 237, 0)
        // Blue: #0047AB (0, 71, 171)
        let r = (255.0 * yellow_ratio) as u8;
        let g = ((237.0 * yellow_ratio) + (71.0 * blue_ratio)) as u8;
        let b = (171.0 * blue_ratio) as u8;

        Ok(Color::new(r, g, b))
    }
}
