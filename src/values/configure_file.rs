use std::io::BufRead;
use std::{fs, io};

use crate::values::configuration_data::CDRef;
use anyhow::Context;
use starlark::environment::GlobalsBuilder;
use starlark::starlark_module;
use starlark::values::none::NoneType;
use starlark::values::Value;

#[starlark_module]
pub fn configure_file_methods(_: &mut GlobalsBuilder) {
    fn configure_file<'v>(
        #[starlark(require = named)] input: Value<'v>,
        #[starlark(require = named)] output: Value<'v>,
        #[starlark(require = named)] configuration: Value<'v>,
    ) -> anyhow::Result<NoneType> {
        let configuration =
            CDRef::from_value(configuration).context("failed to unpack configuration")?;

        let unpack_value = |key: &str| {
            let value = configuration.content.get(key);
            if value.is_none() {
                println!("missing configuration_data {}", key);
                format!("/* #undef {} */", key)
            } else {
                let v = value.unwrap();
                match v.value.get_type() {
                    "string" => format!("#define {} {}", key, v.value.to_repr()),
                    "int" => format!("#define {} {}", key, v.value.to_repr()),
                    "bool" if v.value.to_bool() => format!("#define {}", key),
                    "bool" if !v.value.to_bool() => format!("/* #undef {} */", key),
                    _ => format!("/* #undef {} */", key), // Default to undef for unknown types
                }
            }
        };
        let mut out = String::new();

        let template = fs::File::open(input.to_str()).context("failed to read the input file")?;

        for mut line in io::BufReader::new(template).lines().map_while(Result::ok) {
            if line.starts_with("#cmakedefine") {
                let mut indices = line.match_indices(" ");
                let (key_start, _) = indices.next().context("corrupted cmakedefine")?;
                let (value_start, _) = indices.next().unwrap_or((line.len(), ""));
                let key = &line[key_start + 1..value_start].to_string();
                let value_part = &line[value_start..].trim();

                // Check if the value part contains a @key@ pattern
                if value_part.starts_with('@') && value_part.ends_with('@') {
                    let config_key = &value_part[1..value_part.len() - 1]; // Strip @ symbols
                    line = unpack_value(config_key);
                } else {
                    // Handle existing #cmakedefine with static value (e.g., HAVE_CTIME_S 1)
                    line = unpack_value(key);
                }
            } else if let Some(replacement) = configuration.content.get(&line) {
                line = replacement.value.to_str().to_string();
            }

            out.push_str(&line);
            out.push('\n');
        }

        fs::write(output.to_str(), out)
            .map(|_| NoneType)
            .context("failed to write configuration file")
    }
}
pub fn register_toplevels(globals: &mut GlobalsBuilder) {
    configure_file_methods(globals);
}
