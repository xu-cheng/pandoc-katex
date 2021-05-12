# pandoc-katex

[![Build Status](https://github.com/xu-cheng/pandoc-katex/workflows/build/badge.svg)](https://github.com/xu-cheng/pandoc-katex/actions)
[![Latest Version](https://img.shields.io/crates/v/pandoc-katex.svg)](https://crates.io/crates/pandoc-katex)

A [pandoc filter](https://pandoc.org/filters.html) to render math equations using [KaTeX](https://katex.org).
It is powered by the [`katex` Rust crate](https://github.com/xu-cheng/katex-rs).

## Install

```bash
cargo install pandoc-katex
```

Alternatively, you can download the pre-built binaries from [releases](https://github.com/xu-cheng/pandoc-katex/releases).

## Usage

```bash
pandoc -t html --filter pandoc-katex \
  --css https://cdn.jsdelivr.net/npm/katex@0.13.9/dist/katex.min.css \
  --css https://pandoc.org/demo/pandoc.css \
  --standalone -o output.html /path/to/input.md
```

You can also pass additional flags to custom KaTeX rendering. For example, to use custom LaTeX macro:

```bash
pandoc -t json /path/to/input.md | \
  pandoc-katex --macro '\RR:\mathbb{R}' | \
  pandoc -f json -t html \
    --css https://cdn.jsdelivr.net/npm/katex@0.13.9/dist/katex.min.css \
    --css https://pandoc.org/demo/pandoc.css \
    --standalone -o output.html
```

For more flags, see `pandoc-katex --help`.

## Configure File

Options can also be read from external configure file. The configure file should be in `.toml` format. For example:

```toml
fleqn = true

[macros]
"\\RR" = "\\mathbb{R}"
```

The configure file path can either be passed as a command line argument `--config-file /path/to/config.toml` or set by environment variable `PANDOC_KATEX_CONFIG_FILE`.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
