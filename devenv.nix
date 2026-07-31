{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:
let
  # bevy_cli = import inputs.bevy_cli { system = pkgs.stdenv.system; };
  system = pkgs.stdenv.hostPlatform.system;
in
{
  # https://devenv.sh/packages/
  packages =
    with pkgs;
    [
      inputs.bevy_cli.packages.${system}.bevy
      pkg-config
      nixgl.auto.nixGLDefault
    ]
    ++ lib.optionals (lib.strings.hasInfix "linux" system) [
      # for Linux
      # Audio (Linux only)
      alsa-lib
      # Cross Platform 3D Graphics API
      vulkan-loader
      # For debugging around vulkan
      vulkan-tools
      # Other dependencies
      libudev-zero
      libx11
      libxcursor
      libxi
      libxrandr
      libxkbcommon
      wayland
    ];

  env.LD_LIBRARY_PATH =
    with pkgs;
    lib.makeLibraryPath [
      vulkan-loader
      libx11
      libxi
      libxcursor
      libxkbcommon
      wayland
    ];

  # https://devenv.sh/languages/
  languages.rust = {
    toolchainFile = ./rust-toolchain.toml;
    enable = true;
    mold.enable = true;
    lsp.enable = true;
  };

  overlays = [
    inputs.nixgl.overlay
  ];
}
