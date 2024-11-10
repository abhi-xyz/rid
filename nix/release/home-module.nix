{
pkgs ? import <nixpkgs> { },
...
}:
let
  manifest = (pkgs.lib.importTOML ../../Cargo.toml).package;
in
  {
  options.program.${manifest.name} = {
    enable = pkgs.lib.mkEnableOption "Enable the program";

    package = pkgs.lib.mkOption {
      type = pkgs.lib.types.package;
      default = pkgs.callPackage ./default.nix { };
      description = "The package to use.";
    };
  };
  config = pkgs.lib.mkIf pkgs.config.program.${manifest.name}.enable {
    home.packages = [ pkgs.config.program.${manifest.name}.package ];
  };
}
