{
  description = "intermededit";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    basic = {
      nativeBuildInputs = [
        pkgs.pkg-config
        pkgs.openssl
        pkgs.uv
        pkgs.cloc
        pkgs.gnumake
        pkgs.just
      ];
      shellHook = ''
        export XDG_CACHE_HOME="$(mktemp -d)"
        just
      '';
    };
  in {
    # use `nix develop` if you already have a system-wide installation of Rust
    devShells."x86_64-linux".default = pkgs.mkShell basic;
    # use `nix develop .#full` if you also want Rust
    devShells."x86_64-linux".full = pkgs.mkShell (pkgs.lib.recursiveUpdate basic
      {
        nativeBuildInputs = [
          pkgs.rustc
          pkgs.cargo
        ];
      });
  };
}
