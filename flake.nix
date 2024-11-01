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
          lifecycler = pkgs.rustPlatform.buildRustPackage {
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
              alsa-lib
              udev
            ];
          };

          default = lifecycler;
        };
      });
}
