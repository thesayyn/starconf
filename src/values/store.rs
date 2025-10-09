use anyhow::{anyhow, Result};
use starlark::{any::ProvidesStaticType, eval::Evaluator};
use std::cell::RefCell;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Store {
    pub config_in: String,
    pub config_out: String,
    pub cc_compiler: String,
    pub cc_executable: String,
    pub cc_args: Vec<String>,
    pub dependencies: RefCell<Vec<DDependency>>,
}

impl Store {
    pub fn from_eval<'v>(eval: &mut Evaluator<'v, '_, '_>) -> Result<Self> {
        let store = eval
            .extra
            .ok_or(anyhow!("extra value is not set"))?
            .downcast_ref::<Store>()
            .ok_or(anyhow!("invalid value type"))?;

        Ok(Self {
            config_in: store.config_in.clone(),
            config_out: store.config_out.clone(),
            cc_compiler: store.cc_compiler.clone(),
            cc_executable: store.cc_executable.clone(),
            cc_args: store.cc_args.clone(),
            dependencies: store.dependencies.clone(),
        })
    }
}

// Define the Dependency struct
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DDependency {
    pub name: String,
    pub semver: semver::Version,
}

// Implement custom parsing for Dependency
impl std::str::FromStr for DDependency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid format: '{}'. Expected 'name=semver'", s));
        }

        let name = parts[0].trim();
        let semver = parts[1].trim();

        if name.is_empty() || semver.is_empty() {
            return Err(format!("Name or semver cannot be empty in '{}'", s));
        }

        let ver = semver::Version::parse(semver);
        if ver.is_err() {
            return Err(format!("semver is corrupted '{}'", semver));
        }

        Ok(Self {
            name: name.to_string(),
            semver: ver.unwrap(),
        })
    }
}
