# pandoc-katex

[![Build Status](https://github.com/xu-cheng/pandoc-katex/workflows/build/badge.svg)](https://github.com/xu-cheng/pandoc-katex/actions)
[![Latest Version](https://img.shields.io/crates/v/pandoc-katex.svg)](https://crates.io/crates/pandoc-katex)

A [pandoc filter](https://pandoc.org/filters.html) to render math equations using [KaTeX](https://katex.org).
It is powered by the [`katex` Rust crate](https://github.com/xu-cheng/katex-rs).

## Install

```bash
cargo install pandoc-katex
```

## Usage

```bash
pandoc -t html --filter pandoc-katex \
  --css https://cdn.jsdelivr.net/npm/katex@0.11.1/dist/katex.min.css \
  --css https://pandoc.org/demo/pandoc.css \
  --standalone -o output.html /path/to/input.md
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>
