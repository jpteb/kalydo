{ ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      # declare projects
      nci.toolchainConfig = ./rust-toolchain.toml;
      nci.projects."kalydo" = {
        path = ./.;
        # export all crates (packages and devshell) in flake outputs
        # alternatively you can access the outputs and export them yourself
        export = true;
      };
      # configure crates
      nci.crates = {
        "muxide" = {
          drvConfig = {
            env.HELLO_WORLD = true;
          };
          # look at documentation for more options
        };
        # "my-other-workspace-crate" = {
        #   drvConfig = {
        #     mkDerivation.buildInputs = [pkgs.hello];
        #   };
        #   # look at documentation for more options
        # };
      };
    };
}
