{
  pkgs ? import <nixpkgs> { },
}:

with pkgs;

mkShell rec {
  shellHook = '' 
    export CARGO_HOME=$(pwd)/.cargo
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
  '';

  nativeBuildInputs = [
    clang
    pkg-config
  ];
  buildInputs = [
    (pkgs.fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
      "llvm-tools-preview"
      "rustc-codegen-cranelift-preview"
    ])
    pkgsCross.mingwW64.stdenv.cc
    pkgsCross.mingwW64.windows.pthreads
    rust-analyzer-nightly
    cargo-llvm-cov
    cargo-nextest
    cargo-mutants
    cargo-watch
    cargo-audit
    cargo-deny
    rustup
    grcov
    lld
    mold  
    glib
    glibc
    gdk-pixbuf
    gtk3
    udev
    alsa-lib
    vulkan-loader
    libxkbcommon
    wayland
  ];
  
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  WINIT_UNIX_BACKEND = "wayland";
}
