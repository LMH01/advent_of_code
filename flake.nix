{
  description = "My solutions for the advent of code.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ];
        perSystem =
          { config
          , pkgs
          , system
          , self
          , ...
          }:
          let
            version = "0.0.9";
            craneLib = inputs.crane.mkLib pkgs;
            aocli_src = pkgs.fetchFromGitHub {
              owner = "sncxyz";
              repo = "aocli";
              rev = "v${version}";
              hash = "sha256-3a58HglzJ9aJ3cKxWSLCT2Msn1nlAofAzfwt/YM/p/g=";
            };
            aocli_cargoArtifacts = craneLib.buildDepsOnly { src = aocli_src; };
            aocli = craneLib.buildPackage { cargoArtifacts = aocli_cargoArtifacts; src = aocli_src; };
          in
          {
            devShells.default = pkgs.mkShell {
              buildInputs = with pkgs; [
                cargo
                gcc
                rustfmt
                clippy
                aocli
              ];

              # Certain Rust tools won't work without this
              # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
              # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
              RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            };

          };

      };
}
