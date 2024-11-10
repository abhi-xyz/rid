{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.05";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system}; 
    manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  in {
    devShells.default = import ./nix/shell.nix {};
    packages.${system} = {
      ${manifest.name} = pkgs.callPackage ./nix/release/default.nix { };
      default = self.packages.${system}.${manifest.name};
    };
    homeManagerModules = {
      ${manifest.name} = {
        config,
        pkgs,
        lib,
        ...
        }:
        let
        in
          {
          options.program.${manifest.name} = {
            enable = lib.mkEnableOption "Enable the program";

            package = lib.mkOption {
              type = lib.types.package;
              default = pkgs.callPackage ./nix/release/default.nix { };
              description = "The package to use.";
            };
          };

          config = lib.mkIf config.program.${manifest.name}.enable {
            home.packages = [ config.program.${manifest.name}.package ];
          };
        };
      default = self.homeManagerModules.${manifest.name};
    };
  };
}
