{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        # Fix duplicate instances of these inputs by pointing them to my inputs
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Read from `rust-toolchain.toml` instead of adding `rust-bin.nightly.latest.default` to devShell `buildInputs`
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Libraries that are mostly needed for tauri to work
        libraries = with pkgs; [
          # webkitgtk
          # gtk3
          # cairo
          # gdk-pixbuf
          # glib
          # dbus
          # openssl_3
          # librsvg
        ] ++ pkgs.lib.lists.optionals stdenv.isDarwin [
          # https://github.com/NixOS/nixpkgs/blob/master/pkgs/os-specific/darwin/apple-sdk/frameworks.nix
          # darwin.apple_sdk.frameworks.Cocoa
          # darwin.apple_sdk.frameworks.Kernel
        ];

        packages = with pkgs; [
          # curl
          # wget
          # pkg-config
          # # webkitgtk
          # # gtk3
          # # glib
          # # dbus
          # # openssl_3
          # # librsvg
          # libsoup
          # sqlite

          # nodePackages.pnpm

          # prisma-engines
          # nodePackages.prisma
        ] ++ pkgs.lib.lists.optionals stdenv.isLinux [
          nodejs_20
        ];

        # Inputs needed at compile-time
        nativeBuildInputs = with pkgs; [ rustToolchain ];
        # Inputs needed at runtime
        buildInputs = with pkgs; [ ] ++ packages ++ libraries;
      in
      {
        # packages.default = derivation {
        #   inherit system;
        #   name = "simple";
        #   builder = with pkgs; "${bash}/bin/bash";
        #   args = [ "-c" "echo foo > $out" ];
        #   src = ./.;
        # };

        devShells.default = if  pkgs.stdenv.isLinux then
             pkgs.mkShell {
                  inherit buildInputs nativeBuildInputs;
                  # buildInputs = packages;

                  shellHook = ''
                      PATH="$PWD/node_modules/.bin:$PATH"
                      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
                  '';
                  PRISMA_SCHEMA_ENGINE_BINARY = "${pkgs.prisma-engines}/bin/schema-engine";
                  PRISMA_QUERY_ENGINE_BINARY = "${pkgs.prisma-engines}/bin/query-engine";
                  PRISMA_QUERY_ENGINE_LIBRARY = "${pkgs.prisma-engines}/lib/libquery_engine.node";
                  PRISMA_FMT_BINARY = "${pkgs.prisma-engines}/bin/prisma-fmt";
            }
        else
            pkgs.mkShell { inherit buildInputs nativeBuildInputs; };
      });
}
