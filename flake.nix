{
  inputs = {
    fenix.url = "github:nix-community/fenix/staging";
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, naersk, fenix, utils, ... }:
    let
      buildTargets = {
        "x86_64-linux" = {
          crossSystemConfig = "x86_64-unknown-linux-gnu";
          rustTarget = "x86_64-unknown-linux-gnu";
          makeBuildPackageAttrs = pkgsCross: {
            extraBuildFlags = [
              "-C" "linker=${pkgsCross.clang}/bin/clang" "-C" "link-arg=-fuse-ld=${pkgsCross.mold}/bin/mold"
            ];
          };
        };

        "i686-linux" = {
          crossSystemConfig = "i686-unknown-linux-gnu";
          rustTarget = "i686-unknown-linux-gnu";
          makeBuildPackageAttrs = pkgsCross: {
            extraBuildFlags = [
              "-C" "linker=${pkgsCross.clang}/bin/clang" "-C" "link-arg=-fuse-ld=${pkgsCross.mold}/bin/mold"
            ];
          };
        };

        "aarch64-linux" = {
          crossSystemConfig = "aarch64-unknown-linux-gnu";
          rustTarget = "aarch64-unknown-linux-gnu";
          makeBuildPackageAttrs = pkgsCross: {
            extraBuildFlags = [
              "-C" "linker=${pkgsCross.clang}/bin/clang" "-C" "link-arg=-fuse-ld=${pkgsCross.mold}/bin/mold"
            ];
          };
        };

        "x86_64-windows" = {
          crossSystemConfig = "x86_64-w64-mingw32";
          rustTarget = "x86_64-pc-windows-gnu";
          makeBuildPackageAttrs = pkgsCross: {
            extraBuildFlags = [
              # "-C" "linker=${pkgsCross.stdenv.cc}/bin/${pkgsCross.stdenv.cc.targetPrefix}cc"
              # "-C" "link-arg=-fuse-ld=${pkgsCross.lld}/bin/lld"
            ];
            depsBuildBuild = [
              pkgsCross.stdenv.cc
              pkgsCross.windows.pthreads
            ];
          };
        };
      };

      # eachSystem [system] (system: ...)
      #
      # Returns an attrset with a key for every system in the given array, with
      # the key's value being the result of calling the callback with that key.
      eachSystem = supportedSystems: callback: builtins.foldl'
        (overall: system: overall // { ${system} = callback system; })
        {}
        supportedSystems;

      # eachCrossSystem [system] (buildSystem: targetSystem: ...)
      #
      # Returns an attrset with a key "$buildSystem.$targetSystem" for
      # every combination of the elements of the array of system strings. The
      # value of the attrs will be the result of calling the callback with each
      # combination.
      #
      # There will also be keys "$system.default", which are aliases of
      # "$system.$system" for every system.
      #
      eachCrossSystem = supportedSystems: callback:
        eachSystem supportedSystems (buildSystem: builtins.foldl'
            (inner: targetSystem: inner // {
              "${targetSystem}" = callback buildSystem targetSystem;
            })
            { default = callback buildSystem buildSystem; }
            supportedSystems
        );

      mkPkgs = buildSystem: targetSystem: import nixpkgs ({
        system = buildSystem;
      } // (if targetSystem == null || targetSystem == buildSystem then {} else {
        # The nixpkgs cache doesn't have any packages where cross-compiling has
        # been enabled, even if the target platform is actually the same as the
        # build platform (and therefore it's not really cross-compiling). So we
        # only set up the cross-compiling config if the target platform is
        # different.
        crossSystem.config = buildTargets.${targetSystem}.crossSystemConfig;
      }));

    in {
      packages = eachCrossSystem
        (builtins.attrNames buildTargets)
        (buildSystem: targetSystem: let
          pkgs = mkPkgs buildSystem null;
          pkgsCross = mkPkgs buildSystem targetSystem;
          rustTarget = buildTargets.${targetSystem}.rustTarget;

          fenixPkgs = fenix.packages.${buildSystem};

          toolchain = with fenixPkgs;
          combine [
            minimal.rustc
            minimal.cargo
            (targets.${rustTarget}).latest.rust-std
          ];

          buildPackageAttrs = buildTargets.${targetSystem}.makeBuildPackageAttrs pkgsCross;
          extraBuildFlags =
          if builtins.hasAttr "extraBuildFlags" buildPackageAttrs then
            buildPackageAttrs.extraBuildFlags
          else [];

          naersk-lib = pkgs.callPackage naersk {
            cargo = toolchain;
            rustc = toolchain;
          };

          buildInputs = with pkgs; [
            grcov
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
          
        in
          naersk-lib.buildPackage (buildPackageAttrs // {
            src = ./.;
            strictDeps = true;
            doCheck = false;

            inherit buildInputs;
            nativeBuildInputs = with pkgs; [
              clang
              pkg-config
            ];

            # Builds in / otherwise fsr
            HOME = "$(pwd)";
            
            TARGET_CC = "${pkgsCross.stdenv.cc}/bin/${pkgsCross.stdenv.cc.targetPrefix}cc";
            CARGO_BUILD_TARGET = rustTarget;
            CARGO_BUILD_RUSTFLAGS = [
              "-C" "target-feature=+crt-static"
            ] ++ extraBuildFlags;
          })
        );
          
  }
  
  //

  utils.lib.eachDefaultSystem (
    system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlays.default ];
      };
    in
    {
      nixpkgs.overlays = [ fenix.overlays.default ];
      devShells.default = import ./shell.nix { inherit pkgs; };
    });
}
