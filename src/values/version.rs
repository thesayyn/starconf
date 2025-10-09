use allocative::Allocative;
use anyhow::Context;
use semver::{Version as SemVer, VersionReq};
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::starlark_module;
use starlark::starlark_simple_value;
use starlark::values::starlark_value;
use starlark::values::NoSerialize;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;
use starlark::values::StringValue;
use starlark::values::Value;
use starlark::values::ValueLike;

#[derive(Clone, Debug, derive_more::Display, ProvidesStaticType, NoSerialize, Allocative)]
#[display("<version>")]
pub(crate) struct Version {
    #[allocative(skip)]
    version: SemVer,
}

impl Version {
    pub fn new(ver: &str) -> anyhow::Result<Self> {
        let version = SemVer::parse(ver)?;
        Ok(Self { version })
    }
}

impl From<SemVer> for Version {
    fn from(version: SemVer) -> Self {
        Self { version }
    }
}

#[starlark_module]
fn version_methods(builder: &mut MethodsBuilder) {
    fn version_compare<'v>(
        this: Value<'v>,
        #[starlark(require = pos)] compare: StringValue,
    ) -> starlark::Result<bool> {
        let version = this.downcast_ref::<Version>().unwrap();

        let version_req =
            VersionReq::parse(compare.as_str()).context("failed to parse semver range")?;
        Ok(version_req.matches(&version.version))
    }
}

starlark_simple_value!(Version);

#[starlark_value(type = "version")]
impl<'v> StarlarkValue<'v> for Version {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(version_methods)
    }
}
