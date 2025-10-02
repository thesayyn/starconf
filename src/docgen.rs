use anyhow::{Ok, Result};

use starlark::docs::multipage::{render_markdown_multipage, DocModuleInfo};
use starlark::docs::DocModule;
use starlark::environment::{GlobalsBuilder, LibraryExtension};

mod values;

use std::fs;
use std::path::PathBuf;

fn get_globals() -> DocModule {
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
    globals.build().documentation()
}

fn main() -> Result<()> {
    println!("Generating docs");

    let modules_info = DocModuleInfo {
        module: &get_globals(),
        name: "lib".to_owned(),
        page_path: "lib".to_owned(),
    };

    let res = render_markdown_multipage(vec![modules_info], None);

    fs::remove_dir_all("./docs/lib")?;

    for (k, v) in res {
        let p = PathBuf::from(format!("./docs/{k}.md"));
        if !p.parent().unwrap().exists() {
            fs::create_dir_all(p.parent().unwrap())?;
        }
        eprintln!("docs/{k}.md");
        std::fs::write(format!("./docs/{k}.md"), v).unwrap()
    }
    Ok(())
}
