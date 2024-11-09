{ config, pkgs, lib, ... }:
let
  tomlFormat = pkgs.formats.toml { };
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
in
  {
  options.program.${manifest.name} = {
    enable = lib.mkEnableOption "Enable the ${manifest.name} program";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./default.nix { check = false; };
      description = "The ${manifest.name} package to use.";
    };
    settings = lib.mkOption {
      type = tomlFormat.type;
      default = { };
      example = lib.literalExpression ''
                [directories]
                images_path = "/home/abhi/pics/pictures/images"

                [input]
                dirs = [
                  "/home/abhi/videos",
                ]
      '';
      description = ''
                Configuration written to {file}`$XDG_CONFIG_HOME/${manifest.name}/config.toml`.
      '';
    };
  };
  config = lib.mkIf config.program.${manifest.name}.enable {
    home.packages = [ config.program.${manifest.name}.package ];

    xdg.configFile."${manifest.name}/config.toml" = lib.mkIf (config.program.${manifest.name}.settings != { }) {
      source = tomlFormat.generate "config.toml" config.program.${manifest.name}.settings;
    };
  };
}
