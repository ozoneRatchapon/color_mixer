# Color Mixer

## Overview
Color Mixer is a simple web application built with Rust and Actix Web that demonstrates color mixing principles. Users can add yellow and blue colors through a web interface, and the application will display the resulting mixed color in real-time.

## Features
- **Visual Color Mixing**: Combine yellow and blue colors to see their mixture
- **Real-time Updates**: See the mixed color change instantly as you add colors
- **RESTful API**: Enhanced API endpoints for color manipulation
- **Clean UI**: Minimalist interface with easy-to-use buttons
- **CORS Support**: Support for cross-origin requests
- **RGB-based Color Mixing**: Precise color mixing using exact RGB values
- **Custom Error Handling**: Robust error handling with descriptive messages
- **LRU Cache**: Performance optimization with cached color mixing results

## Screenshots
*(Note: Add actual screenshots of the running application here)*

## Installation

### Prerequisites
- Rust and Cargo (latest stable version)
- Web browser

### Setup

```bash
# Clone the repository
git clone https://github.com/ozoneRatchapon/color_mixer.git

# Navigate to the project directory
cd color_mixer

# Build and run the application
cargo run
```

Once running, open your browser and navigate to:
```
http://127.0.0.1:8080
```

## Usage

1. Click the "Add Yellow" button to add yellow to the mix
2. Click the "Add Blue" button to add blue to the mix
3. Observe the resulting color in the color box below the buttons
4. Continue adding colors to see how the mixture evolves

## Technical Details

### Project Structure

```
color_mixer/
├── src/
│   ├── color_mixer.rs    # Core color mixing logic
│   ├── error.rs          # Custom error types and handling
│   └── main.rs           # Web server and API endpoints
├── static/
│   ├── index.html        # Frontend UI
│   ├── styles.css        # CSS styles for the UI
│   └── scripts.js        # JavaScript for frontend functionality
└── Cargo.toml            # Project dependencies
```

### Color Mixing Algorithm

The application uses a weighted average approach to mix colors:
- Yellow is represented as #FFED00
- Blue is represented as #0047AB
- When mixed, each RGB component is calculated proportionally to the amount of each color added

### API Endpoints

#### POST `/add_color`
Adds a color to the mixer.

**Request Body:**
```json
{
  "color": "yellow" // or "blue"
}
```

**Response:**
```json
{
  "color": "#HEXCODE",       // The resulting mixed color
  "status": "success",        // Operation status
  "rgb": {                   // RGB components
    "r": 255,
    "g": 237,
    "b": 0
  }
}
```

#### GET `/get_current_color`
Retrieves the current color mix.

**Response:**
```json
{
  "color": "#HEXCODE",       // The current mixed color
  "status": "success",
  "rgb": {                   // RGB components
    "r": 127,
    "g": 167,
    "b": 85
  }
}
```

#### POST `/mix_colors`
Mixes specific RGB colors.

**Request Body:**
```json
{
  "color1": {
    "r": 255,
    "g": 237,
    "b": 0
  },
  "color2": {
    "r": 0,
    "g": 71,
    "b": 171
  },
  "ratio": 0.5  // Optional, defaults to 0.5
}
```

**Response:**
```json
{
  "color": "#HEXCODE",       // The resulting mixed color
  "status": "success",
  "rgb": {                   // RGB components
    "r": 127,
    "g": 154,
    "b": 85
  }
}
```

#### POST `/reset`
Resets the color mixer to its initial state.

**Response:**
```json
{
  "status": "success",
  "message": "Color mixer has been reset"
}
```

## Development

### Dependencies
- actix-web: Web server framework
- actix-files: Static file serving
- actix-cors: CORS support for cross-origin requests
- serde: Serialization and deserialization
- serde_json: JSON handling
- thiserror: Custom error type definitions
- lru: LRU cache implementation
- log: Logging facade for Rust
- env_logger: Logger implementation for the log crate

### Building for Production

```bash
cargo build --release
```

The compiled binary will be available in `target/release/color_mixer`.

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Future Improvements

- Add more base colors (red, green, etc.)
- Implement color history to track changes
- Create a more sophisticated color mixing algorithm based on pigment properties
- Add unit tests for color mixing logic
- Implement a Docker container for easy deployment
- Add user authentication for saved color palettes
- Create a mobile-friendly responsive design

## Acknowledgments

- Inspired by basic color theory principles
- Built with the Rust programming language and Actix Web framework
