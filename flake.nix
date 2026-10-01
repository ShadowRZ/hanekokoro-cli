{
  description = "Hanekokoro CLI";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          hanekokoro-cli = pkgs.callPackage ./nix/package.nix {
            lastModifiedDate = self.lastModifiedDate or "19700101";
          };
        in
        {
          default = hanekokoro-cli;
        }
      );
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = import ./nix/dev/shell.nix { inherit pkgs; };
        }
      );
      formatter = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        (import ./nix/dev/formatter.nix { inherit pkgs; })
      );
    };
}
