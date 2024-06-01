{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs;[
    git
    clang
    libclang
    opencv
    cmake
    pkg-config
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
}
