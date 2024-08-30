{
  description = "A generic LSP server using tree-sitter and SQLite";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";

    # Tree-sitter grammars
    tree-sitter-rust = {
      url = "github:tree-sitter/tree-sitter-rust";
      flake = false;
    };
    tree-sitter-python = {
      url = "github:tree-sitter/tree-sitter-python";
      flake = false;
    };
    tree-sitter-javascript = {
      url = "github:tree-sitter/tree-sitter-javascript";
      flake = false;
    };
    tree-sitter-typescript = {
      url = "github:tree-sitter/tree-sitter-typescript";
      flake = false;
    };
    tree-sitter-cpp = {
      url = "github:tree-sitter/tree-sitter-cpp";
      flake = false;
    };
    tree-sitter-c = {
      url = "github:tree-sitter/tree-sitter-c";
      flake = false;
    };
    tree-sitter-go = {
      url = "github:tree-sitter/tree-sitter-go";
      flake = false;
    };
    tree-sitter-java = {
      url = "github:tree-sitter/tree-sitter-java";
      flake = false;
    };
    tree-sitter-ruby = {
      url = "github:tree-sitter/tree-sitter-ruby";
      flake = false;
    };
    tree-sitter-php = {
      url = "github:tree-sitter/tree-sitter-php";
      flake = false;
    };
    tree-sitter-html = {
      url = "github:tree-sitter/tree-sitter-html";
      flake = false;
    };
    tree-sitter-css = {
      url = "github:tree-sitter/tree-sitter-css";
      flake = false;
    };
    tree-sitter-bash = {
      url = "github:tree-sitter/tree-sitter-bash";
      flake = false;
    };
    #tree-sitter-lua = { url = "github:tree-sitter/tree-sitter-lua"; flake = false; };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        # Function to create a derivation for a tree-sitter grammar
        mkTreeSitterGrammar = name: src:
          pkgs.stdenv.mkDerivation {
            pname = "tree-sitter-${name}";
            version = "latest";
            inherit src;
            nativeBuildInputs = with pkgs; [
              tree-sitter
              nodejs
              python3
            ];
            buildPhase = ''
              export HOME=$TMPDIR
              if [ -f binding.gyp ]; then
                npm install
              fi
              tree-sitter generate
            '';
            installPhase = ''
              mkdir -p $out/parser
              if [ -f src/parser.c ]; then
                $CC -o parser.so -I./src src/parser.c src/scanner.c -shared -fPIC -Os -g0 -s
              elif [ -f src/parser.cc ]; then
                $CXX -o parser.so -I./src src/*.cc -shared -fPIC -Os -g0 -s
              else
                echo "No parser source found"
                exit 1
              fi
              cp parser.so $out/parser/${name}.so
            '';
          };

        # Create derivations for all tree-sitter grammars
        treeSitterGrammars =
          builtins.mapAttrs
          (name: src: mkTreeSitterGrammar (builtins.substring 13 (-1) name) src)
          (builtins.removeAttrs inputs ["self" "nixpkgs" "rust-overlay" "flake-utils"]);
      in {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              rustToolchain
              sqlite
              pkg-config
              tree-sitter
              nodejs
              python3
            ]
            ++ (builtins.attrValues treeSitterGrammars);

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

          # Set environment variables for tree-sitter grammar paths
          shellHook = ''
            export TREE_SITTER_GRAMMARS_PATH="${pkgs.lib.makeBinPath (builtins.attrValues treeSitterGrammars)}"
          '';
        };
      }
    );
}
