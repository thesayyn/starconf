use starlark::any::ProvidesStaticType;
use std::cell::RefCell;

#[derive(Debug, ProvidesStaticType, Default)]
pub struct Store {
    pub dependencies: RefCell<Vec<DDependency>>,
}

// Define the Dependency struct
#[derive(Debug, Clone)]
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
