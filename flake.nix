{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs =
    inputs@{
      parts,
      nci,
      treefmt-nix,
      pre-commit-hooks-nix,
      ...
    }:
    let
      tfmt = treefmt-nix;
      pch = pre-commit-hooks-nix;
    in
    parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      imports = [
        nci.flakeModule
        tfmt.flakeModule
        pch.flakeModule
        ./crates.nix
      ];
      perSystem =
        { pkgs, config, ... }:
        let
          # shorthand for accessing outputs
          # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
          outputs = config.nci.outputs;
        in
        {
          treefmt = {
            projectRootFile = "flake.nix";
            programs.rustfmt.enable = true;
            programs.nixfmt.enable = true;
          };

          formatter = config.treefmt.build.wrapper;
          # export the project devshell as the default devshell
          devShells.default = outputs."kalydo".devShell.overrideAttrs (old: {
            packages = with pkgs; (old.packages or [ ]) ++ [ cargo-nextest ];
          });
          # export the release package of one crate as default package
          packages.default = outputs."muxide".packages.release;
        };
    };
}
