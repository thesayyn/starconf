use starlark::{collections::SmallMap, environment::GlobalsBuilder, eval::Arguments, starlark_module, values::Heap};

use super::value::{FrozenCD, CD};


#[starlark_module]
pub fn register_toplevels(globals: &mut GlobalsBuilder) {
   
    #[starlark(
        as_type = FrozenCD,
        speculative_exec_safe,
    )]
    fn configuration_data<'v>(args: &Arguments<'v, '_>, heap: &'v Heap) -> starlark::Result<CD<'v>> {
      Ok(CD { content: SmallMap::new() })
    }
}