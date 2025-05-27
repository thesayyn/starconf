use std::fmt;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

use allocative::Allocative;
use anyhow::Context;
use starlark;
use starlark::any::ProvidesStaticType;
use starlark::environment::GlobalsBuilder;
use starlark::eval::Evaluator;
use starlark::values::list::ListRef;
use starlark::values::none::NoneType;
use starlark::values::tuple::UnpackTuple;
use starlark::values::Coerce;
use starlark::values::FreezeResult;
use starlark::values::FrozenValue;
use starlark::values::StarlarkValue;
use starlark::values::StringValue;
use starlark::values::Value;
use starlark::values::ValueLike;
use starlark::{starlark_complex_value, starlark_module};
use starlark_derive::starlark_value;
use starlark_derive::Freeze;
use starlark_derive::NoSerialize;
use starlark_derive::Trace;

#[derive(Clone, Default, Debug, Trace, Freeze, ProvidesStaticType, Allocative, NoSerialize)]
#[repr(C)]
pub struct ProjectGen<'v, V: ValueLike<'v>> {
    pub(crate) name: V::String,
    pub(crate) version: V::String,
}

impl<'v, V: ValueLike<'v>> Display for ProjectGen<'v, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<project>")
    }
}

unsafe impl<'v> Coerce<ProjectGen<'v, Value<'v>>> for ProjectGen<'static, FrozenValue> {}

#[starlark_value(type = "Project")]
impl<'v, V: ValueLike<'v>> StarlarkValue<'v> for ProjectGen<'v, V> where Self: ProvidesStaticType<'v>
{}

starlark_complex_value!(pub(crate) Project<'v>);

const HEAP_HIDDEN_PROJECT: &str = "#hidden_project#";

#[starlark_module]
pub fn register_autoconconfig_toplevels(_: &mut GlobalsBuilder) {
    fn project_version<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<Value<'v>> {
        let project = eval
            .module()
            .get(HEAP_HIDDEN_PROJECT)
            .ok_or("did you call project() first");
        // thats a lot of unwrap, find a way to turn Result into starlark::Result.
        Ok(project
            .unwrap()
            .downcast_ref::<Project>()
            .unwrap()
            .version
            .to_value())
    }
}

#[starlark_module]
pub(crate) fn register_toplevels(globals: &mut GlobalsBuilder) {
    fn project<'v>(
        #[starlark(require = pos)] name: starlark::values::StringValue<'v>,
        #[starlark(require = named)] version: starlark::values::StringValue<'v>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<NoneType> {
        eval.module().set(
            HEAP_HIDDEN_PROJECT,
            eval.heap().alloc_complex(Project {
                name: name,
                version: version,
            }),
        );
        Ok(NoneType)
    }

    fn add_project_arguments<'v>(
        #[starlark(args)] args: UnpackTuple<Value>,
        #[starlark(require = named)] language: Option<StringValue<'v>>,
    ) -> anyhow::Result<NoneType> {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open("arguments.txt")?;

        for arg in args.into_iter() {
            match arg.get_type() {
                "list" => {
                    for arg in ListRef::from_value(arg)
                        .context("failed to cast to list")?
                        .iter()
                    {
                        assert!(arg.get_type() == "string");
                        file.write((arg.to_str().to_string() + " ").as_bytes())?;
                    }
                }
                "string" => {
                    file.write((arg.to_str().to_string() + " ").as_bytes())?;
                }
                _ => anyhow::bail!("add_project_arguments supports list of strings, or string"),
            }
        }

        Ok(NoneType)
    }
}
