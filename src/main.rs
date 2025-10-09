#![allow(clippy::all)]
mod meson;
mod values;

use clap::Parser;
use clap::ValueHint;
// use meson::translate_to_starlark;

use std::cell::RefCell;
use std::fs;

use starlark::environment::GlobalsBuilder;
use starlark::environment::LibraryExtension;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::syntax::DialectTypes;
use starlark::syntax::{AstModule, Dialect};

use values::store::{DDependency, Store};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    #[clap(short = 'c', long = "config")]
    config: String,

    #[clap(long = "input")]
    config_in: String,

    #[clap(long = "output")]
    config_out: String,

    #[clap(long = "compiler")]
    compiler_type: String,

    #[clap(long = "cc")]
    cc: String,

    #[clap(
        long = "isystem",
        action = clap::ArgAction::Append
    )]
    system_include: Vec<String>,

    #[clap(
        long = "iquote",
        action = clap::ArgAction::Append
    )]
    quote_include: Vec<String>,

    #[clap(
           short = 'd',
           long = "dependency",
           value_name = "NAME=SEMVER",
           value_parser = clap::value_parser!(DDependency),
           value_hint = ValueHint::Other,
           action = clap::ArgAction::Append
       )]
    dependencies: Vec<DDependency>,
}

fn main() {
    let args = CLI::parse();

    let dialect = Dialect {
        enable_def: true,
        enable_f_strings: true,
        enable_keyword_only_arguments: true,
        enable_lambda: true,
        enable_load_reexport: false,
        enable_load: false,
        enable_top_level_stmt: true,
        enable_types: DialectTypes::Enable,
        enable_positional_only_arguments: true,
        ..Default::default()
    };

    let content = fs::read_to_string(&args.config).unwrap();

    let ast = if args.config.ends_with("meson.build") {
        // let tr = translate_to_starlark(&content).unwrap();

        // AstModule::parse("meson.build", tr, &dialect).unwrap()
        panic!("Not supported")
    } else {
        AstModule::parse(&args.config, content, &dialect).unwrap()
    };

    let mut globals = GlobalsBuilder::extended_by(&[
        LibraryExtension::StructType,
        LibraryExtension::RecordType,
        LibraryExtension::EnumType,
        LibraryExtension::NamespaceType,
        LibraryExtension::Map,
        LibraryExtension::Filter,
        LibraryExtension::Partial,
        LibraryExtension::Debug,
        LibraryExtension::Print,
        LibraryExtension::Pprint,
        LibraryExtension::Pstr,
        LibraryExtension::Prepr,
        LibraryExtension::Breakpoint,
        LibraryExtension::Json,
        LibraryExtension::Typing,
        LibraryExtension::CallStack,
        LibraryExtension::SetType,
    ]);
    values::register_toplevels(&mut globals);

    let module = Module::new();

    let mut cc_args = vec![];

    for arg in args.quote_include {
        cc_args.push(format!("-iquote={}", arg));
    }

    for arg in args.system_include {
        cc_args.push(format!("-isystem={}", arg));
    }

    let store = Store {
        dependencies: RefCell::new(args.dependencies),
        cc_compiler: args.compiler_type,
        cc_executable: args.cc,
        cc_args: cc_args,
        config_in: args.config_in,
        config_out: args.config_out,
    };

    {
        let mut eval = Evaluator::new(&module);
        eval.extra = Some(&store);
        let value = eval.eval_module(ast, &globals.build());
        if value.is_err() {
            panic!("{:?}", value.unwrap_err());
        }
    }
}
