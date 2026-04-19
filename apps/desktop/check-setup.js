#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('Checking Zaroxi Desktop setup...\n');

// Check current directory
const currentDir = process.cwd();
const expectedDirName = 'desktop';
const isInDesktopDir = path.basename(currentDir) === expectedDirName;

console.log(`Current directory: ${currentDir}`);
console.log(`In desktop directory: ${isInDesktopDir ? '✅' : '❌'}`);

if (!isInDesktopDir) {
  console.log('\n⚠️  Warning: You should run this from the apps/desktop directory');
  console.log('   Expected to be in: .../apps/desktop/');
  console.log('   Try: cd apps/desktop');
  process.exit(1);
}

// Check package.json
const packageJsonPath = path.join(currentDir, 'package.json');
if (!fs.existsSync(packageJsonPath)) {
  console.log('❌ package.json not found');
  process.exit(1);
}
console.log('✅ package.json found');

// Check node_modules
const nodeModulesPath = path.join(currentDir, 'node_modules');
if (!fs.existsSync(nodeModulesPath)) {
  console.log('⚠️  node_modules not found - run: npm install');
} else {
  console.log('✅ node_modules found');
}

// Check src-tauri
const srcTauriPath = path.join(currentDir, 'src-tauri');
if (!fs.existsSync(srcTauriPath)) {
  console.log('❌ src-tauri directory not found');
  process.exit(1);
}
console.log('✅ src-tauri directory found');

// Check Cargo.toml in src-tauri
const cargoTomlPath = path.join(srcTauriPath, 'Cargo.toml');
if (!fs.existsSync(cargoTomlPath)) {
  console.log('❌ src-tauri/Cargo.toml not found');
  process.exit(1);
}
console.log('✅ src-tauri/Cargo.toml found');

console.log('\n✅ Setup check passed!');
console.log('\nNext steps:');
console.log('1. Install dependencies: npm install');
console.log('2. Build Rust crates: cd ../.. && cargo build --workspace');
console.log('3. Start development: npm run tauri dev');
#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('Checking Zaroxi Desktop setup...\n');

// Check current directory
const currentDir = process.cwd();
const expectedDirName = 'desktop';
const isInDesktopDir = path.basename(currentDir) === expectedDirName;

console.log(`Current directory: ${currentDir}`);
console.log(`In desktop directory: ${isInDesktopDir ? '✅' : '❌'}`);

if (!isInDesktopDir) {
  console.log('\n⚠️  Warning: You should run this from the apps/desktop directory');
  console.log('   Expected to be in: .../apps/desktop/');
  console.log('   Try: cd apps/desktop');
  process.exit(1);
}

// Check package.json
const packageJsonPath = path.join(currentDir, 'package.json');
if (!fs.existsSync(packageJsonPath)) {
  console.log('❌ package.json not found');
  process.exit(1);
}
console.log('✅ package.json found');

// Check node_modules
const nodeModulesPath = path.join(currentDir, 'node_modules');
if (!fs.existsSync(nodeModulesPath)) {
  console.log('⚠️  node_modules not found - run: npm install');
} else {
  console.log('✅ node_modules found');
}

// Check src-tauri
const srcTauriPath = path.join(currentDir, 'src-tauri');
if (!fs.existsSync(srcTauriPath)) {
  console.log('❌ src-tauri directory not found');
  process.exit(1);
}
console.log('✅ src-tauri directory found');

// Check Cargo.toml in src-tauri
const cargoTomlPath = path.join(srcTauriPath, 'Cargo.toml');
if (!fs.existsSync(cargoTomlPath)) {
  console.log('❌ src-tauri/Cargo.toml not found');
  process.exit(1);
}
console.log('✅ src-tauri/Cargo.toml found');

console.log('\n✅ Setup check passed!');
console.log('\nNext steps:');
console.log('1. Install dependencies: npm install');
console.log('2. Build Rust crates: cd ../.. && cargo build --workspace');
console.log('3. Start development: npm run tauri dev');
