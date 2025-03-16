{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, ...} @ inputs:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.fenix.overlays.default ];
        };
      in
      {
        packages.x86_64-linux.default = inputs.fenix.packages.x86_64-linux.minimal.toolchain;
        nixpkgs.overlays = [ inputs.fenix.overlays.default ];
        devShells.default = import ./shell.nix { inherit pkgs; };
      }
    );
}
