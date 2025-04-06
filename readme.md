# Color Mixer

A lightweight and efficient web application for mixing colors, built with Rust and Axum.

## Overview

Color Mixer is an interactive web application that demonstrates color mixing principles through a clean, intuitive interface. Users can mix yellow (#FFED00) and blue (#0047AB) colors, with the application displaying the resulting color in real-time with precise RGB values.

The project showcases modern Rust web development practices, combining a performant backend with a responsive frontend. It features a RESTful API with comprehensive error handling and performance optimizations.

## Features

- Interactive color mixing interface
- Support for mixing yellow (#FFED00) and blue (#0047AB) colors
- Real-time color updates with exact RGB values
- Responsive design with modern UI
- Optimized for performance
- Minimal dependencies
- Comprehensive error handling

## Technical Highlights

- **Rust Performance**: Built with Rust for memory safety and high performance
- **Modern Web Architecture**: Clean separation between backend logic and frontend presentation
- **RESTful API Design**: Well-structured endpoints with JSON communication
- **Efficient Color Algorithms**: Precise RGB-based color mixing with mathematical accuracy
- **Performance Optimization**: CPU-specific optimizations and LTO
- **Error Resilience**: Custom error handling with descriptive messages

## Installation

### Prerequisites
- Rust (latest stable version)
- Cargo (comes with Rust)

### Setup

1. Clone the repository:
```bash
git clone https://github.com/ozoneRatchapon/color_mixer.git
cd color_mixer
```

2. Build the application:
```bash
cargo build --release
```

## Running the Application

To run the application in optimized mode:

```bash
RUSTFLAGS="-C target-cpu=native" cargo run --release
```

The application will be available at `http://localhost:8080`

## Usage

1. Click the "Add Yellow" button to add yellow to the mix
2. Click the "Add Blue" button to add blue to the mix
3. Observe the resulting color in the color box
4. The color counts and total are displayed below
5. The resulting color's hex code is shown at the bottom

## Project Structure

```
color_mixer/
├── src/
│   ├── color_mixer.rs    # Core color mixing logic
│   ├── error.rs          # Custom error types and handling
│   └── main.rs           # Web server and API endpoints
├── static/
│   └── index.html        # Frontend UI with embedded CSS and JS
├── .cargo/
│   └── config.toml       # Build configuration
└── Cargo.toml            # Project dependencies
```

## API Endpoints

### GET `/api/color`
Get the current mixed color.

**Response:**
```json
{
  "color": "#HEXCODE",
  "rgb": [r, g, b]
}
```

### POST `/api/color`
Add a new color to the mixer.

**Request Body:**
```json
{
  "color": "yellow" // or "blue"
}
```

**Response:**
```json
{
  "color": "#HEXCODE",
  "rgb": [r, g, b]
}
```

### POST `/api/clear`
Clear all colors from the mixer.

## Performance Optimizations

The application includes several optimizations:

- CPU-specific optimizations with `target-cpu=native`
- Link Time Optimization (LTO)
- Stripped symbols for smaller binary size
- Minimal dependencies with only essential features
- Optimized build settings in `.cargo/config.toml`
- Efficient color mixing algorithm

## Development

To run in development mode:

```bash
cargo run
```

### Dependencies
- axum: Web server framework
- tower-http: Static file serving
- tokio: Async runtime
- serde: Serialization and deserialization
- thiserror: Custom error type definitions
- rgb: Color handling
- log: Logging facade
- env_logger: Logger implementation

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Commit your changes: `git commit -m 'Add some feature'`
4. Push to the branch: `git push origin feature-name`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Future Improvements

- Add more base colors
- Implement color history
- Add unit tests
- Create a Docker container for deployment
- Add user authentication for saved palettes

## Acknowledgments

- Inspired by basic color theory principles
- Built with the Rust programming language and Axum Web framework
