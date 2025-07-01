<div align="center">
  <img src="public/logo.png" alt="WASM Image Studio Logo" width="200" style="margin-bottom: 5px;">
  <h1>WA-RS Image Studio</h1>
  <p>
    <strong>Web-based image processing application built with Rust, WebAssembly, and Leptos</strong>
  </p>
  <p>
    <a href="#features">Features</a> •
    <a href="#getting-started">Getting Started</a> •
    <a href="#development">Development</a> •
    <a href="#license">License</a>
  </p>
</div>

## 🖼️ Overview

WA-RS Image Studio is a high-performance web application for applying various image filters and effects in real-time. Built with Rust and compiled to WebAssembly, it delivers near-native performance directly in your browser. The application features an intuitive drag-and-drop interface, real-time filter previews, and the ability to export your edited images.

## ✨ Features

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
- **Export Functionality** - Save your edited images in high quality

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or later)
- [Trunk](https://trunkrs.dev/) (Rust WASM web application bundler)

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

3. Install Trunk:
   ```bash
   cargo install trunk wasm-bindgen-cli
   ```

4. Install JavaScript dependencies:
   ```bash
   npm install
   ```

## 🛠️ Development

### Running Locally

Start the development server:

```bash
trunk serve --open
```

This will start the development server and open the application in your default browser. The page will automatically reload if you make changes to the source files.

### Building for Production

To create a production build:

```bash
trunk build --release
```

The build artifacts will be stored in the `dist/` directory.

## 🏗️ Project Structure

```
.
├── Cargo.toml          # Rust project configuration and dependencies
├── index.html          # Main HTML entry point with root element for the WASM app
├── input.css           # Global styles and Tailwind CSS imports
├── public/             # Static assets ----------------------
│   ├── favicon.ico     # Browser tab icon
│   └── logo.png        # Application logo
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