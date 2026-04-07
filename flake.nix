{
  description = "Neote development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
            clang
            lld
          ];

          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy

            # System libraries
            libxkbcommon
            fontconfig
            freetype
            expat
            libglvnd
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            vulkan-loader
            wayland

            # For workspace-daemon file operations
            openssl
          ];

          # Environment variables
          env = {
            # Force X11 backend to avoid Wayland issues
            WINIT_UNIX_BACKEND = "x11";
            # Ensure linker can find libraries
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              libxkbcommon
              fontconfig
              freetype
              expat
              libglvnd
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              vulkan-loader
              wayland
              openssl
            ];
          };

          shellHook = ''
            echo "Neote development environment"
            echo "Run: cargo run --bin desktop"
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "neote";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
          ];

          buildInputs = with pkgs; [
            libxkbcommon
            fontconfig
            freetype
            expat
            libglvnd
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            vulkan-loader
            wayland
            openssl
          ];

          # Force X11 backend
          WINIT_UNIX_BACKEND = "x11";
        };
      }
    );
}
