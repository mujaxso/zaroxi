#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('🔍 Checking Zaroxi Desktop setup...\n');

// Check current directory
const currentDir = process.cwd();
const expectedDirName = 'desktop';
const isInDesktopDir = path.basename(currentDir) === expectedDirName;

console.log(`📁 Current directory: ${currentDir}`);
console.log(`📍 In desktop directory: ${isInDesktopDir ? '✅' : '❌'}`);

if (!isInDesktopDir) {
  console.log('\n⚠️  Warning: You should run this from the apps/desktop directory');
  console.log('   Expected to be in: .../zaroxi/apps/desktop/');
  console.log('   Try: cd apps/desktop');
  process.exit(1);
}

// Check package.json
const packageJsonPath = path.join(currentDir, 'package.json');
if (!fs.existsSync(packageJsonPath)) {
  console.log('❌ package.json not found');
  console.log('   Run: npm init or create package.json');
  process.exit(1);
}
console.log('✅ package.json found');

// Check if package.json has content
const packageJsonContent = fs.readFileSync(packageJsonPath, 'utf8');
if (packageJsonContent.trim() === '' || packageJsonContent.trim() === '{}') {
  console.log('❌ package.json is empty');
  console.log('   Add the necessary configuration');
  process.exit(1);
}
console.log('✅ package.json has content');

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

// Check if Rust is installed
try {
  execSync('rustc --version', { stdio: 'pipe' });
  console.log('✅ Rust is installed');
} catch (error) {
  console.log('❌ Rust is not installed');
  console.log('   Install via: curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh');
}

// Check Node.js version
try {
  const nodeVersion = execSync('node --version', { encoding: 'utf8' }).trim();
  console.log(`✅ Node.js ${nodeVersion}`);
  
  const versionMatch = nodeVersion.match(/v(\d+)/);
  if (versionMatch && parseInt(versionMatch[1]) < 18) {
    console.log('⚠️  Node.js version should be 18+');
  }
} catch (error) {
  console.log('❌ Node.js is not installed');
}

console.log('\n✅ Setup check completed!');
console.log('\n📋 Next steps:');
console.log('1. Install dependencies: npm install');
console.log('2. Build Rust crates: cd ../.. && cargo build --workspace');
console.log('3. Start development: npm run tauri dev');
console.log('\n💡 Quick start:');
console.log('   cd apps/desktop');
console.log('   npm install');
console.log('   cd ../.. && cargo build --workspace');
console.log('   cd apps/desktop && npm run tauri dev');
