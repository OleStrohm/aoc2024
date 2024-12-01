{
  description = "Rust flake with nightly";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, rust-overlay, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-analyzer" "clippy" "rust-src" ];
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          nativeBuildInputs = [ rustToolchain ];
        };
      }
    );
}
