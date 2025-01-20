{
  pkgs ? import <nixpkgs> { },
}:

with pkgs;

mkShell rec {
  shellHook = '' 
    export PATH="/home/$USER/.cargo/bin:$PATH"
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
    rust-analyzer-nightly
    cargo-llvm-cov
    cargo-nextest
    cargo-mutants
    cargo-watch
    cargo-audit
    cargo-deny
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
    # openssl
    # openssl.dev
    # libz
  ];
  
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  WINIT_UNIX_BACKEND = "wayland";
}
