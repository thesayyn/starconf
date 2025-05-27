use starlark::{environment::GlobalsBuilder, starlark_module};

#[starlark_module]
pub(crate) fn register_toplevels(globals: &mut GlobalsBuilder) {
    fn get_option<'v>(
        #[starlark(require = pos)] name: starlark::values::StringValue<'v>,
    ) -> starlark::Result<String> {
        Ok(String::new())
    }

    fn get_option<'v>(
        #[starlark(require = pos)] name: starlark::values::StringValue<'v>,
        #[starlark(require = named)] ty: starlark::values::StringValue<'v>,
    ) -> starlark::Result<String> {
        Ok(String::new())
    }
}
