dev:
  nix develop ./nix/dev --command fish

build:
  cargo build --release

remote-run:
  nix run github:abhi-xyz/rid -- help

release:
  git tag v0.1.2
  git push --tags
