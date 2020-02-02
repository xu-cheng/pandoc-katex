use anyhow::{Context, Result};
use serde_json::{json, Map as JsonMap, Value as JsonValue};

struct Visitor;

impl Visitor {
    #[inline]
    fn new() -> Self {
        Self
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

        let opts = katex::Opts::builder()
            .display_mode(math_type == "DisplayMath")
            .build()
            .unwrap();
        let html = katex::render_with_opts(tex, opts)?;

        obj.clear();
        obj.insert("t".to_owned(), json!("RawInline"));
        obj.insert("c".to_owned(), json!(["html", html]));

        Ok(())
    }

    #[inline]
    fn walk_object(&self, obj: &mut JsonMap<String, JsonValue>) -> Result<()> {
        for mut value in obj.values_mut() {
            self.walk_value(&mut value)?;
        }

        Ok(())
    }

    #[inline]
    fn walk_array(&self, array: &mut Vec<JsonValue>) -> Result<()> {
        for mut value in array.iter_mut() {
            self.walk_value(&mut value)?;
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

fn main() -> Result<()> {
    let mut data: JsonValue = serde_json::from_reader(std::io::stdin().lock())?;
    let visitor = Visitor::new();
    visitor.walk_value(&mut data)?;
    serde_json::to_writer(std::io::stdout().lock(), &data)?;
    Ok(())
}
