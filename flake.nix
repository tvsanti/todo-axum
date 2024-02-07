{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
         nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy 
            rust-analyzer
            cargo-expand
            sqlx-cli
            openssl
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
