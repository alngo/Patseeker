{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
	pkgsCross = {
	  system = "x86_64-linux";
	  crossSystem = {
	    config = "x86_64-w64-mingwW64";
	  };
	};
        devShells.default = with pkgs; mkShell {
          buildInputs = [
	    pkgsCross.mingwW64.stdenv.cc
	    rustup
            openssl
            pkg-config
            eza
            fd
            rust-bin.beta.latest.default
          ];

          RUSTFLAGS = builtins.map (a: "-L ${a}/lib") [
            pkgsCross.mingwW64.windows.pthreads
          ];

          shellHook = ''
            # Aliases
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}

