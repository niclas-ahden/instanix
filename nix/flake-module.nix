{ self, lib, inputs, flake-parts-lib, ... }:

let
  inherit (flake-parts-lib)
    mkPerSystemOption;
in
{
  options = {
    perSystem = mkPerSystemOption
      ({ config, self', inputs', pkgs, system, ... }: {
        options = {
          instanix.overrideCraneArgs = lib.mkOption {
            type = lib.types.functionTo lib.types.attrs;
            default = _: { };
            description = "Override crane args for the instanix package";
          };

          instanix.rustToolchain = lib.mkOption {
            type = lib.types.package;
            description = "Rust toolchain to use for the instanix package";
            default = (pkgs.rust-bin.fromRustupToolchainFile (self + /rust-toolchain.toml)).override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "clippy"
              ];
            };
          };

          instanix.craneLib = lib.mkOption {
            type = lib.types.lazyAttrsOf lib.types.raw;
            default = (inputs.crane.mkLib pkgs).overrideToolchain config.instanix.rustToolchain;
          };

          instanix.src = lib.mkOption {
            type = lib.types.path;
            description = "Source directory for the instanix package";
            # When filtering sources, we want to allow assets other than .rs files
            # TODO: Don't hardcode these!
            default = lib.cleanSourceWith {
              src = self; # The original, unfiltered source
              filter = path: type:
                (lib.hasSuffix "\.html" path) ||
                (lib.hasInfix "/public/" path) ||
                (lib.hasInfix "/style/" path) ||
                # Default filter from crane (allow .rs files)
                (config.instanix.craneLib.filterCargoSources path type)
              ;
            };
          };
        };
        config =
          let
            cargoToml = builtins.fromTOML (builtins.readFile (self + /Cargo.toml));
            inherit (cargoToml.package) name version;
            inherit (config.instanix) rustToolchain craneLib src;

            # Crane builder for cargo-leptos projects
            craneBuild = rec {
              args = {
                inherit src;
                pname = name;
                version = version;
                buildInputs = [
                  pkgs.cargo-leptos
                  pkgs.dart-sass
                  pkgs.binaryen # Provides wasm-opt
                ] ++ lib.optionals pkgs.stdenv.isDarwin [
                  pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
                ];
              };
              cargoArtifacts = craneLib.buildDepsOnly args;
              buildArgs = args // {
                inherit cargoArtifacts;
                buildPhaseCargoCommand = "cargo leptos build --release -vvv";
                cargoTestCommand = "cargo leptos test --release -vvv";
                cargoExtraArgs = "";
                nativeBuildInputs = [
                  pkgs.makeWrapper
                  pkgs.hashdeep # Used to get MD5 of asset dir for cache busting
                ];
                installPhaseCommand = ''
                  mkdir -p $out/bin
                  cp target/release/${name} $out/bin/
                  cp -r target/site $out/bin/
                  siteHash=$(md5deep -r target/site/pkg | cut -f 1 -d " ")
                  pkgDirName="pkg-$siteHash"
                  echo "pkgDirName: $pkgDirName"
                  echo "sourcePath: $out/bin/site/pkg"
                  echo "targetPath: $out/bin/site/$pkgDirName"
                  mv $out/bin/site/pkg $out/bin/site/$pkgDirName
                  wrapProgram $out/bin/${name} \
                    --set LEPTOS_SITE_ROOT $out/bin/site \
                    --set LEPTOS_SITE_PKG_DIR $pkgDirName
                '';
              };
              package = craneLib.buildPackage (buildArgs // config.instanix.overrideCraneArgs buildArgs);

              check = craneLib.cargoClippy (args // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
              });

              doc = craneLib.cargoDoc (args // {
                inherit cargoArtifacts;
              });
            };

            rustDevShell = pkgs.mkShell {
              shellHook = ''
                # For rust-analyzer 'hover' tooltips to work.
                export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
              '';
              buildInputs = [
                pkgs.libiconv
              ];
              nativeBuildInputs = [
                rustToolchain
              ];
            };
          in
          {
            # Rust package
            packages.${name} = craneBuild.package;
            packages."${name}-doc" = craneBuild.doc;

            checks."${name}-clippy" = craneBuild.check;

            # Rust dev environment
            devShells.${name} = pkgs.mkShell {
              inputsFrom = [
                rustDevShell
              ];
              nativeBuildInputs = with pkgs; [
                cargo-leptos
                dart-sass
                binaryen # Provides wasm-opt
              ];
            };
          };
      });
  };
}
