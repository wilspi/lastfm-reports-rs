let
  # Mozilla Overlay
  moz_overlay = import (
    builtins.fetchTarball
      "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz"
  );
  # Update nixpkgs from release: https://github.com/NixOS/nixpkgs/releases/tag/20.03
  nixpkgs = import (builtins.fetchTarball https://github.com/NixOS/nixpkgs/archive/20.03.tar.gz) {
    overlays = [ moz_overlay ];
    config = {};
  };

  frameworks = nixpkgs.darwin.apple_sdk.frameworks;
  rustChannels = (
    nixpkgs.rustChannelOf {
      date = "2020-05-07";
      channel = "stable";
    }
  );

in
  with nixpkgs;

  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [ rustChannels.rust rustChannels.rust-src rustChannels.clippy-preview ];

    nativeBuildInputs = [
      file
      zsh
      wget
      which
      locale
      vim
      less
      htop
      curl
      man
      git
      gitAndTools.diff-so-fancy
      heroku
      openssl
      pkgconfig
      nixpkgs-fmt
      cacert
    ] ++ (
      stdenv.lib.optionals stdenv.isDarwin [
        frameworks.Security
        frameworks.CoreServices
        frameworks.CoreFoundation
        frameworks.Foundation
      ]
    );

    # ENV Variables
    RUST_BACKTRACE = 1;
    HISTFILE = "${toString ./.}/.zsh_history";
    SOURCE_DATE_EPOCH = 315532800;
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
    PROJDIR = "${toString ./.}";

    # Post Shell Hook
    shellHook = ''
      echo "Using ${rustChannels.rust.name}"
    '' + (
      if !pkgs.stdenv.isDarwin then
        ""
      else ''
        # Cargo wasn't able to find CF during a `cargo test` run on Darwin.
        export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
      ''
    ) + ''
      echo "env activated";
    '';
  }
