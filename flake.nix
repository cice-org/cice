{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
    utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, utils,  ... } @ inputs:
     utils.lib.eachDefaultSystem (system:
    let pkgs = nixpkgs.legacyPackages."${system}";
    in
    {
      packages.${system}.devenv-up = self.devShells.${system}.default.config.procfileScript;
      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          ({ pkgs, config, ... }: {
            # This is your devenv configuration
            packages = with pkgs;[
              git
              lldb
              clang
              libclang
              opencv
              cmake
              pkg-config
              protobuf
              # Used by libvnc START
              zlib
              # Used by libvnc END
            ];

            env = {
              LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
            };

            enterShell = ''

            '';

            # https://devenv.sh/tests/
            enterTest = ''

            '';

            # https://devenv.sh/services/
            # services.postgres.enable = true;

            # https://devenv.sh/languages/
            languages = {
              rust = {
                enable = true;
                channel = "nightly";
              };
              c.enable = true;
            };

            # https://devenv.sh/pre-commit-hooks/
            # pre-commit.hooks.shellcheck.enable = true;

            # https://devenv.sh/processes/
            # processes.ping.exec = "ping example.com";

            # See full reference at https://devenv.sh/reference/options/

          })
        ];
      };
    });

}
