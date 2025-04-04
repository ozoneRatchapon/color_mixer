#[derive(Debug, Clone)]
pub enum Color {
    Yellow,
    Blue,
}

#[derive(Debug, Clone)]
pub struct ColorMixer {
    colors: Vec<Color>,
}

impl ColorMixer {
    pub fn new() -> Self {
        Self { colors: Vec::new() }
    }

    pub fn add_color(&mut self, color: Color) {
        self.colors.push(color);
    }

    pub fn get_mixed_color(&self) -> String {
        let mut yellow_count = 0;
        let mut blue_count = 0;

        for color in &self.colors {
            match color {
                Color::Yellow => yellow_count += 1,
                Color::Blue => blue_count += 1,
            }
        }

        match (yellow_count, blue_count) {
            (0, 0) => "#FFFFFF".to_string(),  // white color for no color
            (_y, 0) => "#FFED00".to_string(), // yellow
            (0, _b) => "#0047AB".to_string(), // blue
            (y, b) => {
                let total = y + b;
                let r = (255 * y + 0 * b) / total;
                let g = (237 * y + 71 * b) / total;
                let b = (0 * y + 171 * b) / total;
                format!("#{:02X}{:02X}{:02X}", r, g, b)
            }
        }
    }
}
