{
  description = "Development environment for the Rust programming language";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            gcc
            cmake
            pkg-config
            openssl
            convco
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.SystemConfiguration
            cargo-watch
            sea-orm-cli

            (rust-bin.stable."1.75.0".default.override {
              extensions = [
                "rust-src"
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-analyzer"
              ];
              targets = [ "x86_64-unknown-linux-gnu" ];
            })
          ];
        };
      });
}
