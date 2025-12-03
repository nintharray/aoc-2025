{
  description = "Advent of Code 2025 Day 2 - Rust";
  inputs = {nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";};
  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo
        rust-analyzer
      ];
    };
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "aoc-02";
      version = "0.0.1";
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };
  };
}
