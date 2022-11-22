# pandoc-katex

[![Build Status](https://github.com/xu-cheng/pandoc-katex/workflows/build/badge.svg)](https://github.com/xu-cheng/pandoc-katex/actions)
[![Latest Version](https://img.shields.io/crates/v/pandoc-katex.svg)](https://crates.io/crates/pandoc-katex)

A [pandoc filter](https://pandoc.org/filters.html) to render math equations using [KaTeX](https://katex.org).
It is powered by the [`katex` Rust crate](https://github.com/xu-cheng/katex-rs).

## Install

```bash
cargo install pandoc-katex
```

When building from the source, the following dependencies are required:
* `gcc` and `patch` for Linux, macOS, and MinGW/MSYS2.
* `msvc` for Windows.

Alternatively, you can download the pre-built binaries from [releases](https://github.com/xu-cheng/pandoc-katex/releases).

## Usage

```bash
pandoc -t html --filter pandoc-katex \
  --css https://cdn.jsdelivr.net/npm/katex@0.13.9/dist/katex.min.css \
  --css https://pandoc.org/demo/pandoc.css \
  --standalone -o output.html /path/to/input.md
```

You can also pass additional flags to custom KaTeX rendering. For example, to use custom LaTeX macros:

```bash
pandoc -t json /path/to/input.md | \
  pandoc-katex --macro '\RR:\mathbb{R}' | \
  pandoc -f json -t html \
    --css https://cdn.jsdelivr.net/npm/katex@0.13.9/dist/katex.min.css \
    --css https://pandoc.org/demo/pandoc.css \
    --standalone -o output.html
```

For more flags, see `pandoc-katex --help`.

## Configuration File

Options can also be read from an external configuration file. The configuration file should be in `.toml` format. For example:

```toml
fleqn = true

[macros]
"\\RR" = "\\mathbb{R}"
```

The configuration file path can either be passed as a command line argument `--config-file /path/to/config.toml` or set by environment variable `PANDOC_KATEX_CONFIG_FILE`.

The configuration file accepts the following options. Please refer to <https://katex.org/docs/options.html> for more information.

| Option | Meaning | Accepted values |
|--------|---------|-----------------|
| `output_type` | Set KaTeX output type. | `"html"`, `"mathml"`, `"htmlAndMathml"` |
| `leqno` | Whether to have `\tags` rendered on the left instead of the right. | Boolean |
| `fleqn` | Whether to make display math flush left. | Boolean |
| `throw_on_error` | Whether to let KaTeX throw a `ParseError` for invalid LaTeX. | Boolean |
| `error_color` | Color used for invalid LaTeX. | String |
| `min_rule_thickness` | Minimum thickness, in ems. | Float |
| `max_size` | Max size for user-specified sizes. | Float |
| `max_expand` | Limit for the number of macro expansions. | Int |
| `trust` | Whether to trust users' input. | Boolean |
| `macros` | Custom macros. | Dictionary |

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
