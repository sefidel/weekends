{
  description = "beamy devshell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, flake-utils, rust }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust.overlays.default ];

        pkgs = import nixpkgs { inherit system overlays; };
      in
      rec {
        # packages.example = example;
        # defaultPackage = example;

        # apps.example = flake-utils.lib.mkApp {
        #   drv = packages.example;
        # };


        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain)
            rust-analyzer
            openssl
            pkg-config
          ];
          buildInputs = with pkgs; [
          ] ++ pkgs.lib.optionals (pkgs.stdenv.isDarwin) (with pkgs.darwin.apple_sdk.frameworks; [
            Security
          ]);
        };
      });
}
