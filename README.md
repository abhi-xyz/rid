# rid

`rid` is a lightweight, high-performance alternative to the traditional `rm` command, designed with an emphasis on speed and data preservation. Unlike standard rm, which permanently deletes files, rid safely moves them to a designated "`trash`" directory. This approach enables efficient and fast file removal without risking unintended data loss.

The goal of `rid` is to retain familiarity with `rm`-style syntax while adding useful extra features. Commands that work with `rm` will also work in `rid`, so there’s no need to learn new syntax—just enhanced functionality.

## Is it fast?
Absolutely. File move operations with rid are instantaneous, regardless of file size, since they only adjust file pointers within the filesystem rather than copying bytes across the disk. So, yes its freaking fast.

## Features
- files are moved to `trash dir` instead of being copied and then deleted

## Installation

## Build

```bash
git clone https://github.com/abhi-xyz/rid.git --depth=1 
cd rid
cargo Build --release
# mv target/release/rid ~/local/.bin/ or /bin/

```


### Nixos

- Test run

```bash
nix run github:abhi-xyz/rid -- <COMMAND>

```

- via flake
Flake input
-In flake.nix
```nix
rid = {
    url = "github:abhi-xyz/rid";
    inputs.nixpkgs.follows = "nixpkgs";
};
```

```nix
outputs =
inputs@{
    self,
        rid,
        nixpkgs,
        ...
}
```

```nix
{
    nixosConfigurations."nixos" = nixpkgs.lib.nixosSystem {
    inherit system;
    specialArgs = {
        inherit inputs;
    };
    modules = [
    (
        { ... }:
        {
            nixpkgs.overlays = [ overlay-unstable ];
        }
    )
    ./hosts/configuration.nix
    rid.nixosModules.rid
```
- In configuration.nix
```nix
program.rid.enable = true;
```

## known bugs
- 

## Planned Features
- rid --revert:  Restores files from the trash directory to their original location.
- rid --gc <TIME_PERIOD>:  Performs garbage collection on trashed files, automatically clearing files older than a specified period.
- rid -f:  Forces deletion without moving files to the trash directory.
- rid -s:  Shreds files for secure deletion.
