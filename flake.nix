{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
    unstable-nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, unstable-nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        overlays = [
          (import rust-overlay)
          (final: prev: {
            unstable = import unstable-nixpkgs {
              inherit system;
              config.allowUnfree = true;
            };
          })
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        {
        devShells.default = import ./nix/shell.nix { inherit pkgs; };
        packages = {
          ${manifest.name} = pkgs.callPackage ./nix/default.nix { };
          default = pkgs.callPackage ./nix/default.nix { };
        };
        homeManagerModules.rid = import ./nix/module.nix { inherit pkgs; };
        homeManagerModules.default = self.homeManagerModules.rid;
      }
    );
}
