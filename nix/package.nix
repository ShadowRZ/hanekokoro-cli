{
  lib,
  rustPlatform,
  lastModifiedDate ? "19700101",
}:

let
  cargo = lib.importTOML ../Cargo.toml;
  stableVersion = cargo.workspace.package.version;
  version =
    let
      year = lib.substring 0 4 lastModifiedDate;
      month = lib.substring 4 2 lastModifiedDate;
      day = lib.substring 6 2 lastModifiedDate;
    in
    "${stableVersion}-unstable-${year}-${month}-${day}";
in
rustPlatform.buildRustPackage {
  pname = "hanekokoro-cli";
  inherit version;

  src = ./..;

  cargoLock.lockFile = ../Cargo.lock;

  meta = {
    homepage = "https://github.com/ShadowRZ/hanekokoro-cli";
    description = "@ShadowRZ's Nix/NixOS/Nixpkgs helpers";
    license = lib.licenses.gpl3Plus;
    mainProgram = "hkk";
    platforms = lib.platforms.unix;
  };
}
