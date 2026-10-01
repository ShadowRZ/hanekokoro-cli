{
  inputs = {
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      # Use flake = false because we're going to import manually
      flake = false;
    };
    flake-compat = {
      url = "github:NixOS/flake-compat";
      flake = false;
    };
  };

  outputs = _: { };
}
