{...}: {
  languages.nix.enable = true;
  languages.rust.enable = true;
  languages.rust.channel = "nightly";

  pre-commit.hooks.deadnix.enable = true;
  pre-commit.hooks.clippy.enable = true;
}
