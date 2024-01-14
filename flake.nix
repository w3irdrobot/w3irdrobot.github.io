{
  description = "blog flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        inputs = [
          pkgs.hugo
          pkgs.just
        ];
      in
      {
        devShell = pkgs.mkShell {
          packages = inputs;
        };
      }
    );
}
