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
          ];

          buildInputs = with pkgs; [
            # Rust toolchain
            rustc
            cargo
            rustfmt
            clippy

            # Essential system libraries
            # For windowing (both X11 and Wayland)
            libxkbcommon
            # X11 fallback (important for compatibility)
            libX11
            libXcursor
            libXi
            libXrandr
            # Wayland (optional, but good to have)
            wayland
            # Graphics
            libglvnd
            # Fonts
            fontconfig
            freetype
            # Other
            openssl
            # For file dialogs - minimal GTK dependencies
            gtk3
            glib
            # xdg-desktop-portal for Wayland file dialogs
            xdg-desktop-portal
          ];

          # Environment variables
          env = {
            # Ensure linker can find libraries
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              # Essential libraries
              libxkbcommon
              # X11
              libX11
              libXcursor
              libXi
              libXrandr
              # Wayland
              wayland
              # Graphics
              libglvnd
              # Fonts
              fontconfig
              freetype
              # GTK for file dialogs
              gtk3
              glib
              # Other
              openssl
            ];
            # For Wayland file dialogs
            # This helps rfd work better on Wayland
            GTK_USE_PORTAL = "1";
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
            # Essential libraries
            libxkbcommon
            fontconfig
            freetype
            libglvnd
            libX11
            libXcursor
            libXi
            libXrandr
            wayland
            openssl
            # For file dialogs
            gtk3
            glib
          ];

          # Don't force any backend - let winit choose the appropriate one
          # This allows both X11 and Wayland to work
        };
      }
    );
}
