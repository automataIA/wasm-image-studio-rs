<div align="center">
  <img src="./public/logo.png" alt="WASM Image Studio Logo" width="200" style="margin-bottom: 5px;">
  <h1>WASM Image Studio</h1>
  <p>
    <strong>Web-based image processing application built with Rust, WebAssembly, Leptos, and DaisyUI</strong>
  </p>
  <p>
    <a href="#features">Features</a> •
    <a href="#tech-stack">Tech Stack</a> •
    <a href="#getting-started">Getting Started</a> •
    <a href="#development">Development</a>
  </p>
</div>

## 🖼️ Overview

WASM Image Studio is a high-performance web application for applying various image filters and effects in real-time. Built with Rust and compiled to WebAssembly, it delivers near-native performance directly in your browser. The application features an intuitive drag-and-drop interface, real-time filter previews, and the ability to export your edited images.

## ✨ Features

- **Modern UI** - Clean, responsive interface built with DaisyUI and Tailwind CSS
- **Dark/Light Mode** - Toggle between themes with a single click
- **Drag & Drop Interface** - Easily upload images by dragging and dropping them onto the canvas
- **Real-time Filter Previews** - See changes instantly as you adjust filter settings
- **Multiple Filter Types**
  - Grayscale
  - Sepia
  - Blur
  - Brightness adjustment
  - Contrast adjustment
  - Hue rotation
  - Saturation adjustment
  - Invert colors
  - Edge detection
  - Sharpen
- **Adjustable Intensity** - Fine-tune each filter with precision controls
- **Responsive Design** - Works on both desktop and mobile devices
- **Export Functionality** - Save your edited images in multiple formats (PNG, JPEG, WEBP)
- **Accessible** - Built with accessibility in mind

## 🛠️ Tech Stack

- **Frontend Framework**: [Leptos](https://leptos.dev/) (v0.8) - Full-stack web framework for Rust
- **UI Components**: [DaisyUI](https://daisyui.com/) (v4.12.0) - Clean, customizable component library
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) (v3.4.3) - Utility-first CSS framework
- **Image Processing**: `photon-rs` - High-performance image processing in Rust
- **Build Tool**: [Trunk](https://trunkrs.dev/) - WASM web application bundler
- **Language**: Rust with WebAssembly for near-native performance

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version, 1.70+ recommended)
- [Node.js](https://nodejs.org/) (v18 or later)
- [Trunk](https://trunkrs.dev/) (Rust WASM web application bundler)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) (for WebAssembly interop)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/wasm-image-studio-rs.git
   cd wasm-image-studio-rs
   ```

2. Install Rust WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Trunk and wasm-bindgen:
   ```bash
   cargo install trunk wasm-bindgen-cli
   ```

4. Install Node.js dependencies:
   ```bash
   npm install
   ```

## 🚧 Development

### Running Locally

Start the development server with hot-reloading:

```bash
trunk serve --open
```

The application will be available at `http://localhost:3000` and will automatically reload when you make changes.

### Building for Production

Create an optimized production build:

```bash
trunk build --release
```

The production-ready files will be in the `dist/` directory.

### Project Structure

```
wasm-image-studio-rs/
├── src/
│   ├── components/     # Reusable UI components
│   │   ├── drag_drop.rs      # File upload component
│   │   ├── filter_controls.rs # Filter controls panel
│   │   ├── image_canvas.rs    # Image display and processing
│   │   └── export_panel.rs    # Image export controls
│   │
│   ├── pages/          # Application pages
│   │   └── home.rs     # Main application page
│   │
│   ├── filters/        # Image filter implementations
│   ├── lib.rs         # Library root
│   └── main.rs        # Application entry point
│
├── public/            # Static assets
├── styles/            # Global styles
└── index.html         # Main HTML entry point
```

### Styling Guidelines

- Use Tailwind CSS utility classes for styling
- Follow DaisyUI component patterns where possible
- Maintain consistent spacing using Tailwind's spacing scale
- Use semantic HTML elements with appropriate ARIA attributes
- Follow the project's color scheme defined in `tailwind.config.js`

### Testing

Run the test suite with:

```bash
cargo test
```

For end-to-end testing:
```bash
wasm-pack test --headless --firefox
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for code formatting
- Run `clippy` for linting

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Leptos](https://leptos.dev/) for the awesome Rust web framework
- [Photon-rs](https://github.com/silvia-odwyer/photon) for image processing
- [Trunk](https://trunkrs.dev/) for WASM application bundling
- All contributors who have helped improve this project

## 🏗️ Project Structure

```
.
# Root
├── Cargo.toml          # Rust project configuration and dependencies
├── Cargo.lock          # Lock file for Rust dependencies
├── index.html          # Main HTML entry point with root element for the WASM app
├── input.css           # Global styles and Tailwind CSS imports
├── Trunk.toml          # Trunk configuration
├── info/               # Project documentation
│   └── structure.md    # Detailed project structure documentation

# Source Code
├── src/
│   ├── components/     # Reusable UI components
│   │   ├── app_layout.rs    # Main application layout
│   │   ├── counter_btn.rs   # Counter button component
│   │   ├── drag_drop.rs     # File upload component
│   │   ├── export_panel.rs  # Image export controls
│   │   ├── filter_controls.rs # Filter adjustment controls
│   │   ├── image_canvas.rs  # Image display and manipulation
│   │   └── mod.rs           # Components module exports
│   │
│   ├── filters/        # Image processing filters
│   │   ├── core.rs     # Core filter implementations
│   │   ├── mod.rs      # Filters module exports
│   │   └── wrappers.rs # WebAssembly bindings
│   │
│   ├── pages/          # Application pages
│   │   ├── home.rs     # Main application page
│   │   ├── not_found.rs # 404 error page
│   │   └── mod.rs      # Pages module exports
│   │
│   ├── utils/          # Utility functions
│   │   ├── memory.rs   # Memory management utilities
│   │   └── mod.rs      # Utils module exports
│   │
│   ├── lib.rs          # Library entry point
│   └── main.rs         # Application entry point

# Static Assets
├── public/             # Static assets
│   ├── favicon.ico     # Browser tab icon
│   └── logo.png        # Application logo

# Build Output
├── target/             # Compiled artifacts (gitignored)
└── dist/               # Production build output (gitignored)
├── src/ ----------------
│   ├── components/     # Reusable UI components -------------
│   │   ├── app_layout.rs      # Main application layout with header and content area
│   │   ├── counter_btn.rs     # Example counter button component (demo)
│   │   ├── drag_drop.rs       # Drag and drop file upload component
│   │   ├── export_panel.rs    # Controls for exporting processed images
│   │   ├── filter_controls.rs # UI controls for adjusting filter parameters
│   │   ├── image_canvas.rs    # Canvas component for displaying and processing images
│   │   └── mod.rs # Module exports for components
│   │
│   ├── filters/        # Image processing functionality -----
│   │   ├── core.rs     # Core filter implementations using photon-rs
│   │   ├── wrappers.rs # WebAssembly bindings for JavaScript interop
│   │   └── mod.rs      # Filter type definitions and public API
│   │
│   ├── pages/          # Application pages and routing ------
│   │   ├── home.rs     # Main application page
│   │   ├── not_found.rs# 404 error page
│   │   └── mod.rs      # Page routing and exports
│   │
│   ├── utils/          # Utility functions ------------------
│   │   ├── memory.rs   # Memory management utilities
│   │   └── mod.rs      # Utility module exports
│   │
│   ├── lib.rs          # Library entry point
│   └── main.rs         # Application entry point
│
├── tailwind.config.js  # Tailwind CSS configuration
└── Trunk.toml          # Trunk (WASM bundler) configuration
```

### Key Components Breakdown

- **src/components/**: Contains reusable UI components built with Leptos
  - `drag_drop.rs`: Handles file uploads via drag-and-drop or file selection
  - `filter_controls.rs`: Provides sliders and controls for adjusting filter parameters
  - `image_canvas.rs`: Renders and processes images using HTML5 Canvas
  - `export_panel.rs`: Manages image export functionality and format options

- **src/filters/**: Implements image processing operations
  - `core.rs`: Contains the actual filter implementations using photon-rs
  - `wrappers.rs`: Provides WebAssembly bindings for JavaScript interop
  - Supports various filters: grayscale, sepia, blur, brightness, contrast, etc.

- **src/pages/**: Handles application routing and page layouts
  - `home.rs`: Main application interface with the image editor
  - `not_found.rs`: 404 error page

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Leptos](https://leptos.dev/), a full-stack web framework for Rust
- Powered by [photon-rs](https://github.com/silvia-odwyer/photon) for image processing
- Styled with [Tailwind CSS](https://tailwindcss.com/)
- Bundled with [Trunk](https://trunkrs.dev/)