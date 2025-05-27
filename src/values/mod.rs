use starlark::environment::GlobalsBuilder;

mod autoconfig;
mod compiler;
mod configuration_data;
mod configure_file;
mod dependency;
mod host_machine;
mod option;
mod project;
pub(crate) mod store;
mod version;

pub fn register_toplevels(builder: &mut GlobalsBuilder) {
    project::register_toplevels(builder);
    configuration_data::register_toplevels(builder);
    host_machine::register_toplevels(builder);
    configure_file::register_toplevels(builder);
    dependency::register_toplevels(builder);
    option::register_toplevels(builder);
    builder.namespace("autoconfig", |builder| {
        autoconfig::register_autoconfig_toplevels(builder);
        project::register_autoconconfig_toplevels(builder);
        compiler::register_autoconconfig_toplevels(builder);
    });
}
