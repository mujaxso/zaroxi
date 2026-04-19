# Zaroxi Desktop App

This is the Tauri 2-based desktop application for Zaroxi Studio.

## Prerequisites

- Node.js 18+ and npm
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Tauri CLI: `npm install -g @tauri-apps/cli`

## Setup

1. Navigate to this directory:
   ```bash
   cd apps/desktop
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the Rust dependencies:
   ```bash
   cd ../..
   cargo build --workspace
   ```

## Development

Run the app in development mode:
```bash
cd apps/desktop
npm run tauri dev
```

This will start:
- Frontend development server on http://localhost:1420
- Tauri application with hot reload

## Building for Production

```bash
cd apps/desktop
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Project Structure

- `src/` - React TypeScript frontend
- `src-tauri/` - Rust backend
- `public/` - Static assets

## Troubleshooting

### Common Issues

1. **npm install fails**: Make sure you're in the `apps/desktop` directory
2. **Rust dependencies not found**: Run `cargo build --workspace` from the root
3. **Tauri not found**: Install globally with `npm install -g @tauri-apps/cli`

### Development Tips

- Use `npm run dev` for frontend-only development (without Tauri)
- Use `npm run tauri dev` for full application with Rust backend
- Check browser console for frontend errors
- Check terminal for Rust backend errors
