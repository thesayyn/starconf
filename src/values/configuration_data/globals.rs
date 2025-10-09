use starlark::{collections::SmallMap, environment::GlobalsBuilder, starlark_module};

use super::value::{FrozenCD, CD};

#[starlark_module]
pub fn register_toplevels(globals: &mut GlobalsBuilder) {
    #[starlark(
        as_type = FrozenCD,
        speculative_exec_safe,
    )]
    fn configuration_data<'v>() -> starlark::Result<CD<'v>> {
        Ok(CD {
            content: SmallMap::new(),
        })
    }
}
