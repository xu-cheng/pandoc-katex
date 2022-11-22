use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use serde::Deserialize;
use serde_json::{json, Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Visitor {
    katex_opts: katex::Opts,
}

impl Visitor {
    fn new(katex_opts: katex::Opts) -> Self {
        Self { katex_opts }
    }

    fn visit_object(&self, obj: &mut JsonMap<String, JsonValue>) -> Result<()> {
        if obj.get("t").map(|v| v == "Math") != Some(true) {
            return self.walk_object(obj);
        }

        let array = obj.get("c").context("failed to read field `c`")?;
        let math_type = array
            .get(0)
            .context("failed to read math type")?
            .get("t")
            .context("failed to read field `t`")?;
        let tex = array
            .get(1)
            .context("faild to read tex code")?
            .as_str()
            .context("invalid data type")?;

        let mut opts = self.katex_opts.clone();
        opts.set_display_mode(math_type == "DisplayMath");
        let html = katex::render_with_opts(tex, opts)?;

        obj.clear();
        obj.insert("t".to_owned(), json!("RawInline"));
        obj.insert("c".to_owned(), json!(["html", html]));

        Ok(())
    }

    #[inline]
    fn walk_object(&self, obj: &mut JsonMap<String, JsonValue>) -> Result<()> {
        for value in obj.values_mut() {
            self.walk_value(value)?;
        }

        Ok(())
    }

    #[inline]
    fn walk_array(&self, array: &mut [JsonValue]) -> Result<()> {
        for value in array.iter_mut() {
            self.walk_value(value)?;
        }

        Ok(())
    }

    fn walk_value(&self, value: &mut JsonValue) -> Result<()> {
        match value {
            JsonValue::Array(array) => self.walk_array(array)?,
            JsonValue::Object(obj) => self.visit_object(obj)?,
            _ => {}
        }

        Ok(())
    }
}

/// Options read from config file.
/// Read <https://katex.org/docs/options.html> for more information.
#[derive(Debug, Deserialize)]
struct ConfigOpt {
    /// Set KaTeX output type. Accepted values: html, mathml, htmlAndMathml.
    output_type: Option<String>,
    /// Whether to have `\tags` rendered on the left instead of the right.
    leqno: Option<bool>,
    /// Whether to make display math flush left.
    fleqn: Option<bool>,
    /// Whether to let KaTeX throw a `ParseError` for invalid LaTeX.
    throw_on_error: Option<bool>,
    /// Color used for invalid LaTeX.
    error_color: Option<String>,
    /// Minimum thickness, in ems.
    min_rule_thickness: Option<f64>,
    /// Max size for user-specified sizes.
    max_size: Option<f64>,
    /// Limit for the number of macro expansions.
    max_expand: Option<i32>,
    /// Whether to trust users' input.
    trust: Option<bool>,
    /// Custom macros.
    macros: Option<HashMap<String, String>>,
}

impl ConfigOpt {
    fn load_from_file(file: &Path) -> Result<Self> {
        Ok(toml::from_str(fs::read_to_string(&file)?.as_str())?)
    }
}

/// Options read from arguments.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct ArgOpt {
    /// Set KaTeX output type. Accepted values: html, mathml, htmlAndMathml
    #[clap(long, value_parser = parse_output_type)]
    output_type: Option<katex::OutputType>,

    /// Make `\tags` rendered on the left instead of the right.
    #[clap(long)]
    leqno: bool,

    /// Make display math flush left.
    #[clap(long)]
    fleqn: bool,

    /// Make KaTeX throw a ParseError for invalid LaTeX.
    #[clap(long)]
    throw_on_error: bool,

    /// Color used for invalid LaTeX.
    #[clap(long)]
    error_color: Option<String>,

    /// Minimum thickness, in ems.
    #[clap(long)]
    min_rule_thickness: Option<f64>,

    /// Max size for user-specified sizes.
    #[clap(long)]
    max_size: Option<f64>,

    /// Limit for the number of macro expansions.
    #[clap(long)]
    max_expand: Option<i32>,

    /// Trust users' input.
    #[clap(long)]
    trust: bool,

    /// Use custom marco. e.g. `-m '\RR:\mathbb{R}'`.
    #[clap(short = 'm', long = "macro")]
    macros: Vec<String>,

    /// Load KaTeX options from external `.toml` file.
    #[clap(
        short = 'f',
        long = "config-file",
        env = "PANDOC_KATEX_CONFIG_FILE",
        value_parser
    )]
    config_file: Option<PathBuf>,

    /// Pandoc output format. This argument is ignored.
    #[clap(name = "OUTPUT_FORMAT")]
    #[allow(dead_code)]
    output_format: Option<String>,
}

fn parse_output_type(input: &str) -> Result<katex::OutputType> {
    match input {
        "html" => Ok(katex::OutputType::Html),
        "mathml" => Ok(katex::OutputType::Mathml),
        "htmlAndMathml" => Ok(katex::OutputType::HtmlAndMathml),
        _ => bail!("invalid katex output type {}", input),
    }
}

impl ArgOpt {
    fn get_katex_opts(&self) -> Result<katex::Opts> {
        let mut opts = katex::Opts::default();

        if let Some(config_file) = &self.config_file {
            let cfg_opt = ConfigOpt::load_from_file(config_file)?;

            if let Some(output_type) = &cfg_opt.output_type {
                opts.set_output_type(parse_output_type(output_type)?);
            }

            if let Some(leqno) = cfg_opt.leqno {
                opts.set_leqno(leqno);
            }

            if let Some(fleqn) = cfg_opt.fleqn {
                opts.set_fleqn(fleqn);
            }

            if let Some(throw_on_error) = cfg_opt.throw_on_error {
                opts.set_throw_on_error(throw_on_error);
            }

            if let Some(error_color) = cfg_opt.error_color {
                opts.set_error_color(error_color);
            }

            if let Some(min_rule_thickness) = cfg_opt.min_rule_thickness {
                opts.set_min_rule_thickness(min_rule_thickness);
            }

            if let Some(max_size) = cfg_opt.max_size {
                opts.set_max_size(Some(max_size));
            }

            if let Some(max_expand) = cfg_opt.max_expand {
                opts.set_max_expand(Some(max_expand));
            }

            if let Some(trust) = cfg_opt.trust {
                opts.set_trust(trust);
            }

            if let Some(macros) = cfg_opt.macros {
                for (macro_name, macro_body) in macros.into_iter() {
                    opts.add_macro(macro_name, macro_body);
                }
            }
        }

        if let Some(output_type) = self.output_type {
            opts.set_output_type(output_type);
        }

        if self.leqno {
            opts.set_leqno(true);
        }

        if self.fleqn {
            opts.set_fleqn(true);
        }

        if self.throw_on_error {
            opts.set_throw_on_error(true);
        }

        if let Some(error_color) = &self.error_color {
            opts.set_error_color(error_color.clone());
        }

        if let Some(min_rule_thickness) = self.min_rule_thickness {
            opts.set_min_rule_thickness(min_rule_thickness);
        }

        if let Some(max_size) = self.max_size {
            opts.set_max_size(Some(max_size));
        }

        if let Some(max_expand) = self.max_expand {
            opts.set_max_expand(Some(max_expand));
        }

        if self.trust {
            opts.set_trust(true);
        }

        for m in &self.macros {
            let mut split = m.splitn(2, ':');
            let macro_name = split
                .next()
                .ok_or_else(|| anyhow!("invalid macro entry '{}'", m))?
                .to_string();
            let macro_body = split
                .next()
                .ok_or_else(|| anyhow!("invalid macro entry '{}'", m))?
                .to_string();
            opts.add_macro(macro_name, macro_body);
        }

        Ok(opts)
    }
}

fn main() -> Result<()> {
    let opt = ArgOpt::parse();
    let katex_opts = opt.get_katex_opts()?;

    let mut data: JsonValue = serde_json::from_reader(std::io::stdin().lock())?;
    let visitor = Visitor::new(katex_opts);
    visitor.walk_value(&mut data)?;
    serde_json::to_writer(std::io::stdout().lock(), &data)?;
    Ok(())
}
