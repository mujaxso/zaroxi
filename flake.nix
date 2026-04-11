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
            libX11
            libXcursor
            libXi
            libXrandr
            vulkan-loader
            wayland

            # For workspace-daemon file operations
            openssl
            
            # D-Bus for xdg-desktop-portal (RFD xdg-portal feature)
            dbus

            # xdg-desktop-portal for RFD file dialogs (xdg-portal feature)
            # Note: xdg-desktop-portal needs to be running at runtime
            # These are development libraries for building
            glib
            gtk3  # Still needed for some dependencies
            pango
            atk
            gdk-pixbuf
            xdg-desktop-portal
            # Hyprland-specific portal implementation
            xdg-desktop-portal-hyprland
            gsettings-desktop-schemas  # For GTK3 settings
          ];

          # Environment variables
          env = {
            # Force X11 backend to avoid Wayland issues
            WINIT_UNIX_BACKEND = "x11";
            # Set GDK backend to x11 for GTK3
            GDK_BACKEND = "x11";
            # GTK3 theme settings for Nix environment
            GTK_THEME = "Adwaita";
            GTK_DATA_PREFIX = "${pkgs.gtk3}";
            # Ensure GTK can find its modules
            GTK_PATH = "${pkgs.gtk3}/lib/gtk-3.0:${pkgs.gtk3}/lib/gtk-3.0/3.0.0";
            # Additional GTK environment variables
            XDG_DATA_DIRS = "${pkgs.gtk3}/share:${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}";
            GI_TYPELIB_PATH = "${pkgs.gtk3}/lib/girepository-1.0";
            # Ensure linker can find libraries
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
              libxkbcommon
              fontconfig
              freetype
              expat
              libglvnd
              libX11
              libXcursor
              libXi
              libXrandr
              vulkan-loader
              wayland
              openssl
              # GTK3 dependencies for RFD
              glib
              gtk3
              pango
              atk
              gdk-pixbuf
              # D-Bus may still be needed
              dbus
              # Hyprland portal
              xdg-desktop-portal-hyprland
            ];
          };

          shellHook = ''
            echo "Neote development environment"
            echo "Run: cargo run --bin desktop"
            echo ""
            echo "Note: For file picker in Hyprland, ensure xdg-desktop-portal-hyprland is running:"
            echo "  systemctl --user status xdg-desktop-portal-hyprland"
            echo "If not running, start it with:"
            echo "  systemctl --user start xdg-desktop-portal-hyprland"
            echo ""
            echo "Also ensure xdg-desktop-portal is running:"
            echo "  systemctl --user status xdg-desktop-portal"
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
            # For GLib/GTK3
            wrapGAppsHook
          ];

          buildInputs = with pkgs; [
            libxkbcommon
            fontconfig
            freetype
            expat
            libglvnd
            libX11
            libXcursor
            libXi
            libXrandr
            vulkan-loader
            wayland
            openssl
            # GTK3 dependencies for RFD
            glib
            gtk3
            pango
            atk
            gdk-pixbuf
            gsettings-desktop-schemas  # For GTK3 settings
            # D-Bus may still be needed
            dbus
            # Hyprland portal
            xdg-desktop-portal-hyprland
          ];

          # Force X11 backend
          WINIT_UNIX_BACKEND = "x11";
          # Set GDK backend to x11 for GTK3
          GDK_BACKEND = "x11";
        };
      }
    );
}
