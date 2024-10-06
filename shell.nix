let
  nixpkgsVer = "7069932e560daa85506f65ec7f63e4bbc5e0d22a";
  pkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/${nixpkgsVer}.tar.gz") {
    config = {};
    overlays = [
      (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  };
  libs = with pkgs; [
    openssl
  ];
in pkgs.mkShell {
  name = "mental-instability-bot";

  buildInputs = libs ++ (with pkgs; [
    (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
    gcc
    pkg-config
    cmake
    perl
    go
    ninja
  ]);

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath libs;
  DATABASE_URL = "postgres://mental_instability_bot:CHANGEMEPLEASE@localhost/mental_instability_bot";
}
