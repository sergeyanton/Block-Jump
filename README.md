# Block Jump - WebAssembly Game Project

A simple block-jumping game built with Rust, WebAssembly, and TypeScript as part of COSC473 Wasm Widget Assignment.

## Overview

Block Jump is a browser-based game that demonstrates the integration of Rust code compiled to WebAssembly (WASM) with a TypeScript frontend. The game logic is implemented in Rust inside the `wasm-crate` directory, while the browser interface is managed with TypeScript.

## Project Structure

- `/src` - TypeScript frontend code
- `/wasm-crate` - Rust code for game logic that compiles to WebAssembly
- `/public/wasm` - Compiled WebAssembly files and JavaScript bindings

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) and npm
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Installation

1. Clone the repository
2. Install dependencies:
   ```
   npm install
   ```
3. Build the WebAssembly module:
   ```
   cd wasm-crate
   wasm-pack build --target web --out-dir ../public/wasm
   ```
4. Start the development server:
   ```
   npm run dev
   ```
5. Open your browser and navigate to http://localhost:5173/

## Development

- The game logic is in `wasm-crate/src/game.rs`
- Spikes implementation can be found in `wasm-crate/src/spike.rs`
- The Rust library entry point is `wasm-crate/src/lib.rs`

## Building for Production

To create a production build:

```
npm run build
```

This will compile the TypeScript files and optimize the Rust/WebAssembly code for production.

## Technologies Used

- Rust
- WebAssembly
- TypeScript
- Vite
