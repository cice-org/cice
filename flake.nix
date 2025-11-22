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
      pkgsCross = pkgs.pkgsCross;
      # 通过 Fenix 定义包含 miri 的 nightly 工具链[1](@ref)
      rust-toolchain = fenix.packages.${system}.combine [
        (fenix.packages.${system}.fromToolchainFile { file = ./rust-toolchain.toml; })
        fenix.packages.${system}.targets.x86_64-pc-windows-gnu.latest.rust-std
      ];
    in
    {
      packages.${system}.devenv-up = self.devShells.${system}.default.config.procfileScript;
      devShells.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          ({ pkgs, config, ... }: {
            overlays = [ fenix.overlays.default ];
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
              # rust-toolchain  # 注入自定义工具链
              # Windows cross-compilation
              pkgsCross.mingwW64.stdenv.cc
              pkgsCross.mingwW64.windows.pthreads
            ];

            env = {
              LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
              # Windows cross-compilation
              # CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "${pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
              # CC_x86_64_pc_windows_gnu = "${pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-gcc";
              # CXX_x86_64_pc_windows_gnu = "${pkgsCross.mingwW64.stdenv.cc}/bin/x86_64-w64-mingw32-g++";
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
                toolchain = rust-toolchain;
                targets = [
                  "x86_64-pc-windows-gnu"
                ];
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
