ts-parsers

- parsers-info format:
```json
{
    "language": "rust",
    "url": "https://github.com/MDeiml/tree-sitter-markdown",
    "files": ["src\/parser.c", "src\/scanner.c"],
    "location": "tree-sitter-markdown",
    "revision": "23d9cb2ce2f4d0914e7609b500c5fc8dfae0176f",
    "install_from_grammar": false
}
```

subcommands
- compile [LANGUAGE]:
    - -c/--compiler: (will default to clang) clang or zig
    - -t/--target: (only available if target is zig) [possible values: x86_64-linux, aarch64-linux, x86_64-macos, aarch64-macos, x86_64-windows]
    - -d/--destination: (will default to `$NVIM_DATA/tree-sitter/parser`)
    - -a/--all: compile all parsers
    - -r/--release: pick a specific nvim-treesitter-parsers release
    - --no-backup: (will default to false) don't create a zip backup of working parsers

- download
    - --latest: download the latest complied parsers
    - --release: download a specific release
    - --location: (will default to `$NVIM_DATA/treesitter`)
    - --no-backup: (will default to false) don't create a zip backup of working parsers

- list: 
    - --downloads: list all available parsers for download
    - --backups: list all available backup/restore points

- restore: restore the `tree-sitter` folder to a previous version (backup folder names are timestamps)
