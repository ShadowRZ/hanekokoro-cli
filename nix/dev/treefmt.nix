{ ... }:
{
  projectRootFile = "flake.nix";

  programs = {
    keep-sorted.enable = true;
    nixfmt.enable = true;
    rustfmt.enable = true;
    shfmt.enable = true;
  };
}
