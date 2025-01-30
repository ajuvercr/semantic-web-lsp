# Semantic Web Language Server

[![CI](https://github.com/ajuvercr/semantic-web-lsp/actions/workflows/ci.yml/badge.svg)](https://github.com/ajuvercr/semantic-web-lsp/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-latest-blue)](https://ajuvercr.github.io/semantic-web-lsp/docs/lsp_core/index.html)
![LICENSE](https://img.shields.io/badge/License-MIT-8A2BE2)
[![Visual Studio Marketplace Last Updated](https://img.shields.io/visual-studio-marketplace/last-updated/ajuvercr.semantic-web-lsp?label=VSCode%20Extension)](https://marketplace.visualstudio.com/items?itemName=ajuvercr.semantic-web-lsp)

This repo includes the source code for the semantic web language server.
The language server provides IDE like functionality for semantic web languages, including Turtle, JSON-LD and SPARQL.

A live demo can be found [online](https://ajuvercr.github.io/semantic-web-lsp/), built with monaco editors.

## Documentation

- [lsp-core](https://ajuvercr.github.io/semantic-web-lsp/docs/lsp_core/index.html)
- [lang-turtle](https://ajuvercr.github.io/semantic-web-lsp/docs/lang_turtle/index.html)
- [lang-jsonld](https://ajuvercr.github.io/semantic-web-lsp/docs/lang_jsonld/index.html)
- [lang-sparql](https://ajuvercr.github.io/semantic-web-lsp/docs/lang_sparql/index.html)
- [lsp-bin](https://ajuvercr.github.io/semantic-web-lsp/docs/lsp_bin/index.html)
- [lsp-web](https://ajuvercr.github.io/semantic-web-lsp/docs/lsp_web/index.html)


## Features

### Diagnostics

- Syntax diagnostics
- Undefined prefix diagnostics
- SHACL shape diagnostics

### Completion

- Prefix completion (just start writing the prefix, `foa` completes to `foaf:` and adding the prefix statement)
- Property completion (ordered according to domain)
- Class completion (when writing the object where the prediate is `a`)

### Hover

- Shows additional information about the entities like class

### Rename

- Rename terms local to the current file 

### Formatting

- Format Turtle

### Highlighting

- Enables semantic highlighting


## Use the LSP

Currently a fluwent install is possible for NeoVim and VSCode.
However the language server protocol enables swift integration into other editors.

### VS Code

Install the semantic web lsp extension ([vscode](https://marketplace.visualstudio.com/items?itemName=ajuvercr.semantic-web-lsp) or [open-vscode](https://open-vsx.org/extension/ajuvercr/semantic-web-lsp)).
The extension starts the lsp from WASM and starts the vscode LSP client.

### NeoVim

To use the LSP you will always have to install the binary.
So do that first:

```
cargo install --git https://github.com/ajuvercr/semantic-web-lsp --bin lsp-bin
```
Or locally
```
git clone https://github.com/ajuvercr/semantic-web-lsp.git
cargo install --path lsp-bin
```

Configure the LSP in NeoVim.

```lua
#  Add a config to lspconfig.configs
local configs = require("lspconfig.configs")

configs.jsonld = {
  default_config = {
    cmd = { 'lsp-bin' },
    filetypes = { 'jsonld', 'turtle', 'sparql' },
    root_dir = require("lspconfig.util").find_git_ancestor,
    single_file_support = true,
    init_options = {},
  }
}

# Start the LSP
local lspconfig = require("lspconfig")

lspconfig.jsonld.setup {
  on_attach = M.on_attach,
  capabilities = M.capabilities,
}
```


## Screenshots

|Undefined prefix|Shape violation|
|---|---|
| ![Undefined Prefixes](./screenshots/undefined_prefix.png) | ![Shape violations](./screenshots/shape.png) |

|Complete Class|Complete Property|
|---|---|
| ![Complete Class](./screenshots/complete_class.png) | ![Complete Property](./screenshots/complete_property.png) |
