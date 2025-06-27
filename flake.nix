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

  outputs = { self, nixpkgs, devenv, utils, fenix, ... } @ inputs:
     utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages."${system}";
      # 通过 Fenix 定义包含 miri 的 nightly 工具链[1](@ref)
      rust-toolchain = fenix.packages.${system}.combine [
        fenix.packages.${system}.latest.rustc
        fenix.packages.${system}.latest.cargo
        fenix.packages.${system}.latest.miri
        fenix.packages.${system}.latest.rust-src  # Miri 需要源码分析[1](@ref)
      ];
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
              # Used by libvnc START
              zlib
              # Used by libvnc END
              rust-toolchain  # 注入自定义工具链
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
                toolchain = rust-toolchain;
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
