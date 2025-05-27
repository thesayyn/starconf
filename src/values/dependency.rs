use std::convert::Infallible;
use std::fmt;
use std::fmt::Display;

use allocative::Allocative;
use starlark;
use starlark::any::ProvidesStaticType;
use starlark::environment::GlobalsBuilder;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::eval::Evaluator;
use starlark::values::none::NoneOr;
use starlark::values::none::NoneType;
use starlark::values::Coerce;
use starlark::values::FreezeResult;
use starlark::values::FrozenValue;
use starlark::values::Heap;
use starlark::values::StarlarkValue;
use starlark::values::StringValue;
use starlark::values::UnpackValue;
use starlark::values::Value;
use starlark::values::ValueLike;
use starlark::{starlark_complex_value, starlark_module};
use starlark_derive::starlark_value;
use starlark_derive::Freeze;
use starlark_derive::NoSerialize;
use starlark_derive::Trace;

use crate::values::store::Store;
use crate::values::version::Version;

#[derive(Clone, Debug, Trace, Freeze, ProvidesStaticType, Allocative, NoSerialize)]
#[repr(C)]
pub struct DependencyGen<'v, V: ValueLike<'v>> {
    pub(crate) name: V::String,
    pub(crate) version: V,
    pub(crate) found: bool,
}

impl<'v, V: ValueLike<'v>> Display for DependencyGen<'v, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<dependency>")
    }
}

unsafe impl<'v> Coerce<DependencyGen<'v, Value<'v>>> for DependencyGen<'static, FrozenValue> {}

#[starlark_value(type = "Dependency")]
impl<'v, V: ValueLike<'v>> StarlarkValue<'v> for DependencyGen<'v, V>
where
    Self: ProvidesStaticType<'v>,
{
    fn get_methods() -> Option<&'static Methods> {
        get_dependency_methods()
    }

    fn iterate_collect(&self, _heap: &'v Heap) -> starlark::Result<Vec<Value<'v>>> {
        Ok(vec![])
    }
}

impl<'v> UnpackValue<'v> for Dependency<'v> {
    type Error = Infallible;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        // TODO: remove cloned
        Ok(Dependency::from_value(value).cloned())
    }
}

starlark_complex_value!(pub(crate) Dependency<'v>);

pub(super) fn get_dependency_methods() -> Option<&'static Methods> {
    static RES: MethodsStatic = MethodsStatic::new();
    RES.methods(dependency_methods)
}

#[starlark_module]
pub(crate) fn dependency_methods(registry: &mut MethodsBuilder) {
    fn found<'v>(this: Dependency<'v>) -> anyhow::Result<bool> {
        Ok(this.found)
    }

    fn version<'v>(
        this: Dependency<'v>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<Value<'v>> {
        Ok(this.version)
    }

    fn partial_dependency<'v>(
        this: Dependency<'v>,
        #[starlark(require = named, default = false)] compile_args: Value<'v>,
        #[starlark(require = named, default = false)] link_args: Value<'v>,
        #[starlark(require = named, default = false)] links: Value<'v>,
        #[starlark(require = named, default = false)] includes: Value<'v>,
        #[starlark(require = named, default = false)] sources: Value<'v>,
    ) -> anyhow::Result<Dependency<'v>> {
        Ok(this)
    }
}

#[starlark_module]
pub(crate) fn register_toplevels(globals: &mut GlobalsBuilder) {
    fn dependency<'v>(
        #[starlark(require = pos)] name: Value<'v>,
        #[starlark(require = named)] disabler: Option<bool>,
        #[starlark(require = named)] required: Option<Value<'v>>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> anyhow::Result<Value<'v>> {
        let store = eval
            .extra
            .unwrap()
            .downcast_ref::<Store>()
            .unwrap()
            .dependencies
            .borrow();
        let found_dep = store.iter().find(|dd| dd.name == name.to_str());
        if found_dep.is_none() && required.is_some_and(|r| r.to_bool()) {
            anyhow::bail!("missing dependency {}", name.to_string())
        }
        if found_dep.is_none() {
            println!(
                "missing dependency {}, available: {:?}",
                name.to_str(),
                store.iter().map(|d| &d.name).collect::<Vec<&String>>()
            )
        }
        Ok(eval.heap().alloc_complex(Dependency {
            name: eval.heap().alloc_str(name.to_str().as_str()),
            found: found_dep.is_some(),
            version: eval.heap().alloc_simple(
                found_dep.map_or(Version::new("0.0.0").unwrap(), |dep| {
                    Version::from(dep.semver.clone())
                }),
            ),
        }))
    }
}
