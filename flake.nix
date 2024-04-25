{
  description = "Simple permutation cipher tool";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        name = "permutate";

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "permutate";
          version = "0.1.0";

          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };

        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.rust.packages.stable.rustPlatform.rustLibSrc
          ];
          nativeBuildInputs = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rust-analyzer
            pkgs.clippy
            pkgs.rustfmt
            pkgs.lldb
          ];
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
