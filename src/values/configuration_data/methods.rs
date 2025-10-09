use starlark::{
    environment::MethodsBuilder,
    eval::Evaluator,
    starlark_module,
    values::{none::NoneType, Value},
};

use crate::values::configuration_data::value::ValueAndDescription;

use super::refs::CDMut;

#[starlark_module]
pub(crate) fn configuration_data_methods(registry: &mut MethodsBuilder) {
    fn set<'v>(
        this: Value<'v>,
        #[starlark(require = pos)] key: String,
        #[starlark(require = pos)] value: Value<'v>,
        #[starlark(require = named)] description: Option<Value<'v>>,
    ) -> starlark::Result<NoneType> {
        let mut this = CDMut::from_value(this)?;
        this.aref
            .content
            .insert(key, ValueAndDescription { description, value });
        Ok(NoneType)
    }
    fn set10<'v>(
        this: Value<'v>,
        #[starlark(require = pos)] key: Value<'v>,
        #[starlark(require = pos)] value: Value<'v>,
        #[starlark(require = named)] description: Option<Value<'v>>,
        eval: &mut Evaluator<'v, '_, '_>,
    ) -> starlark::Result<NoneType> {
        let mut this = CDMut::from_value(this)?;
        let key = key.get_hashed()?;
        let value = eval.heap().alloc(i32::from(value.to_bool()));
        this.aref.content.insert(
            key.to_str().to_string(),
            ValueAndDescription { value, description },
        );
        Ok(NoneType)
    }
}
