{
  description = "Spin Nix Devshel";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... } @inputs:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };

          rustTarget = pkgs.rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
            extensions = [ "rust-src" "rust-analyzer"];
            targets = [ "wasm32-wasi" "wasm32-unknown-unknown"];
          });

        in
        {

          devShells.default = pkgs.mkShell {

            # Extra inputs can be added here
            nativeBuildInputs = with pkgs; [
              rustTarget
              pkg-config
              cargo-leptos
              binaryen
              dart-sass
              gcc
            ];
            RUST_SRC_PATH = "${rustTarget}/lib/rustlib/src/rust/library";
            #CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.mold}/bin/mold";
          };
        });
}
