{
  pkgs ? import <nixpkgs> { },
}:

let
  flake = (
    import (
      let
        lock = builtins.fromJSON (builtins.readFile ./flake.lock);
        nodeName = lock.nodes.root.inputs.flake-compat;
      in
      fetchTarball {
        url =
          lock.nodes.${nodeName}.locked.url
            or "https://github.com/NixOS/flake-compat/archive/${lock.nodes.${nodeName}.locked.rev}.tar.gz";
        sha256 = lock.nodes.${nodeName}.locked.narHash;
      }
    ) { src = ./.; }
  );
  treefmt-lib = import flake.outputs.inputs.treefmt-nix;
  treefmt = treefmt-lib.evalModule pkgs ./treefmt.nix;
  rustfmt' = pkgs.rustfmt.override { asNightly = true; };
in

pkgs.mkShell {
  packages = [
    pkgs.rustc
    pkgs.cargo
    pkgs.clippy
    pkgs.rust-analyzer
    rustfmt'
  ];

  nativeBuildInputs = [
    # keep-sorted start
    pkgs.cargo-audit
    pkgs.cargo-bloat
    pkgs.cargo-deny
    pkgs.cargo-edit
    pkgs.cargo-release
    pkgs.keep-sorted
    pkgs.nixd
    pkgs.nixfmt
    pkgs.shfmt
    pkgs.tombi
    # keep-sorted end
    treefmt.config.build.wrapper
  ];
}
