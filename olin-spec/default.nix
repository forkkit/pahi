{ sources ? import ../nix/sources.nix, pkgs ? import sources.nixpkgs { }
, dhall-lang ? sources.dhall-lang, dhall ? import ../nix/dhall.nix }:
pkgs.stdenv.mkDerivation rec {
  name = "olin-spec";
  version = "latest";
  src = ./.;
  phases = "buildPhase installPhase";

  buildInputs = [ dhall.dhall-simple dhall-lang ];

  buildPhase = ''
    buildDir=$(pwd)
    (cd $src/errors && dhall text < renderErrorMD.dhall) > errors.md
    mkdir ns
    (cd $src/ns && dhall text < log.dhall > $buildDir/ns/log.md)
    (cd $src/ns && dhall text < random.dhall > $buildDir/ns/random.md)
    (cd $src/ns && dhall text < runtime.dhall > $buildDir/ns/runtime.md)
    (cd $src/ns && dhall text < time.dhall > $buildDir/ns/time.md)
  '';

  installPhase = ''
    mkdir -p $out/docs/olin-spec
    cp -rf $src/README.md $out/docs/olin-spec/README.md
    cp -rf errors.md $out/docs/olin-spec/errors.md
    cp -rf ns $out/docs/olin-spec/
  '';
}