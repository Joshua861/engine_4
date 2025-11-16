{
  pkgs ? import <nixpkgs> { },
}:
let
  libPath =
    with pkgs;
    lib.makeLibraryPath [
      # Graphics libraries
      libGL
      mesa
      vulkan-loader
      vulkan-validation-layers
      # Wayland/X11 support
      libxkbcommon
      wayland
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
      alsa-lib
      # Additional system libraries that might be needed
      libglvnd
      stdenv.cc.cc.lib
    ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Graphics development packages
    vulkan-tools
    vulkan-headers
    pkg-config
    # System libraries
    libGL
    mesa
    vulkan-loader
    vulkan-validation-layers
    libxkbcommon
    wayland
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    # ALSA development libraries
    alsa-lib
    alsa-utils
    alsa-tools
    alsa-oss
    linuxKernel.packages.linux_6_15.perf
  ];
  # Environment variables
  RUST_LOG = "debug";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  LD_LIBRARY_PATH = libPath;
  # Vulkan environment
  VK_LAYER_PATH = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
  shellHook = ''
      echo "wgpu development environment loaded"
      echo "Available graphics backends:"
      echo "- Vulkan: $(if command -v vulkaninfo >/dev/null 2>&1; then echo "✓"; else echo "✗"; fi)"
      echo "- OpenGL: $(if command -v glxinfo >/dev/null 2>&1; then echo "✓"; else echo "✗"; fi)"
    export RUST_LOG=warn
      alias cr="cargo run"
      alias crr="cargo run --release"
      alias ca="cargo add"
  '';
}
