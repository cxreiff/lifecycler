{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages = rec {
          lifecycler = pkgs.rustPlatform.buildRustPackage rec {
            name = "lifecycler";
            src = pkgs.lib.cleanSource ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
              outputHashes = {
                "zune-jpeg-0.4.11" = "sha256-Iks3Gslg+LcGIWQL2K3SfGTKxamlYv8SiRfq14kW/pE=";
              };
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];
            buildInputs = with pkgs; [
              udev
              alsa-lib
              vulkan-loader
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              xorg.libXrandr
              libxkbcommon
              wayland
            ];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
          };

          default = lifecycler;
        };
      });
}
