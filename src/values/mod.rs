use starlark::{
    environment::GlobalsBuilder, eval::Evaluator, starlark_module,
    values::starlark_value_as_type::StarlarkValueAsType,
};

use crate::values::store::Store;

mod compiler;
mod configuration_data;
mod configure_file;
mod host_machine;
// mod option;
// mod dependency;
// mod project;
// mod version;
pub(crate) mod store;

#[starlark_module]
pub fn type_toplevels(_: &mut GlobalsBuilder) {
    const compiler: StarlarkValueAsType<compiler::Compiler> = StarlarkValueAsType::new();
}

#[starlark_module]
pub fn register_autoconf_toplevels(_: &mut GlobalsBuilder) {
    fn get_compiler<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<compiler::Compiler> {
        let store = Store::from_eval(eval)?;
        Ok(compiler::Compiler {
            name: store.cc_compiler,
            args: store.cc_args,
            executable: store.cc_executable,
        })
    }
}

#[starlark_module]
pub fn register_fn_toplevels(_: &mut GlobalsBuilder) {
    fn config_in<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<String> {
        let store = Store::from_eval(eval)?;
        Ok(store.config_in)
    }
    fn config_out<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<String> {
        let store = Store::from_eval(eval)?;
        Ok(store.config_out)
    }
}

pub fn register_toplevels(builder: &mut GlobalsBuilder) {
    // project::register_toplevels(builder);
    // option::register_toplevels(builder);
    // project::register_autoconconfig_toplevels(builder);
    // dependency::register_toplevels(builder);
    configuration_data::register_toplevels(builder);
    host_machine::register_toplevels(builder);
    configure_file::register_toplevels(builder);
    builder.namespace("autoconf", |builder| {
        register_autoconf_toplevels(builder);
    });
    register_fn_toplevels(builder);
}
