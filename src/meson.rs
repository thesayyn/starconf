use anyhow::{Context, Result};
use indent::indent_all_by;
use tree_sitter::{Node, Parser};

#[inline]
fn visit_string(node: Node<'_>, source: &[u8], out: &mut String) {
    out.push_str(node.utf8_text(source).unwrap());
}

#[inline]
fn visit_bool(node: Node<'_>, source: &[u8], out: &mut String) {
    let val = node.utf8_text(source).unwrap();
    if val == "true" {
        out.push_str("True");
    } else {
        out.push_str("False");
    }
}

#[inline]
fn visit_number(node: Node<'_>, source: &[u8], out: &mut String) {
    out.push_str(node.utf8_text(source).unwrap());
}

#[inline]
fn visit_ident(node: Node<'_>, source: &[u8], out: &mut String) {
    let ident = node.utf8_text(source).unwrap();
    if ident == "meson" {
        out.push_str("autoconfig");
    } else {
        out.push_str(node.utf8_text(source).unwrap());
    }
}

#[inline]
fn visit_variableunit(node: Node<'_>, source: &[u8], out: &mut String) {
    visit_node(node.child(0).unwrap(), source, out);
}

#[inline]
fn visit_operatorunit(node: Node<'_>, source: &[u8], out: &mut String) {
    dbg!(node.to_sexp());
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[inline]
fn visit_var_unit(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn visit_recursive(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

#[inline]
fn visit_source_file(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            out.push('\n');
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn visit_if_condition(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            out.push('\n');
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn visit_if_command(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();

    if cursor.goto_first_child() {
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(' ');
        visit_node(cursor.node(), source, out);
        out.push_str(":");
        if !cursor.goto_next_sibling() {
            return;
        }
        let mut sout = String::new();
        loop {
            sout.push_str("\n  ");
            visit_node(cursor.node(), source, &mut sout);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        out.push_str(indent_all_by(2, sout).as_str());
    }
}

fn visit_else_command(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();

    if cursor.goto_first_child() {
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(':');
        let mut sout = String::new();
        loop {
            sout.push_str("\n  ");
            visit_node(cursor.node(), source, &mut sout);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        out.push_str(indent_all_by(2, sout).as_str());
    }
}

fn visit_foreach_command(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();

    if cursor.goto_first_child() {
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(' ');
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(' ');
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(' ');
        visit_node(cursor.node(), source, out);
        if !cursor.goto_next_sibling() {
            return;
        }
        out.push(':');
        let mut sout = String::new();
        loop {
            sout.push_str("\n  ");
            visit_node(cursor.node(), source, &mut sout);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        out.push_str(indent_all_by(2, sout).as_str());
    }
}

fn visit_list(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn visit_dict(node: Node<'_>, source: &[u8], out: &mut String) {
    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            visit_node(cursor.node(), source, out);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

fn visit_node(node: Node<'_>, source: &[u8], out: &mut String) {
    // dbg!(node.kind());
    match node.kind() {
        "string" => visit_string(node, source, out),
        "bool" => visit_bool(node, source, out),
        "number" => visit_number(node, source, out),

        "list" => visit_list(node, source, out),
        "dictionaries" => visit_dict(node, source, out),

        "source_file" => visit_source_file(node, source, out),
        "normal_command" => visit_recursive(node, source, out),
        "pair" => visit_recursive(node, source, out),
        "var_unit" => visit_var_unit(node, source, out),
        "variableunit" => visit_variableunit(node, source, out),
        "operatorunit" => visit_operatorunit(node, source, out),
        "identifier" => visit_ident(node, source, out),

        "expression_statement" => visit_recursive(node, source, out),
        "if_condition" => visit_if_condition(node, source, out),
        "if_command" => visit_if_command(node, source, out),
        "elseif_command" => visit_if_command(node, source, out),
        "else_command" => visit_else_command(node, source, out),
        "foreach_command" => visit_foreach_command(node, source, out),

        "if" => out.push_str("if"),
        "elif" => out.push_str("elif"),
        "else" => out.push_str("else"),
        "foreach" => out.push_str("for"),

        ":" if node.parent().is_some_and(|n| n.kind() == "foreach_command") => out.push_str("in"),
        ":" if node
            .parent()
            .is_some_and(|n| n.parent().is_some_and(|n| n.kind() == "dictionaries")) =>
        {
            out.push_str(": ")
        }
        ":" if node.parent().is_some_and(|n| n.kind() == "pair") => out.push_str(" = "),

        "." => out.push('.'),
        "," => out.push_str(", "),
        "(" | ")" => out.push_str(node.kind()),
        "[" | "]" => out.push_str(node.kind()),
        "{" | "}" => out.push_str(node.kind()),
        "=" | "+=" | "==" | "!=" => {
            out.push(' ');
            out.push_str(node.kind());
            out.push(' ');
        }
        ">" | ">=" | "<=" | "<" => {
            out.push(' ');
            out.push_str(node.kind());
            out.push(' ');
        }
        "and" | "or" | "not" | "in" => {
            out.push(' ');
            out.push_str(node.kind());
            out.push(' ');
        }

        "keyword_break" => out.push_str("break"),
        "keyword_continue" => out.push_str("continue"),

        "endif" => {}
        "endforeach" => {}
        "comment" => {}

        x => {
            let r = node.range();
            panic!(
                "ty: {} {} at meson.build:{}:{} at {}:{}",
                x,
                node.to_sexp(),
                r.start_point.row,
                r.start_point.column,
                r.end_point.row,
                r.end_point.column
            )
        }
    }
}

pub(crate) fn translate_to_starlark(code: &String) -> Result<String> {
    let mut parser = Parser::new();
    let language = tree_sitter_meson::LANGUAGE;
    parser
        .set_language(&language.into())
        .context("error loading meson parser")?;

    let tree = parser.parse(code, None).unwrap();
    let mut out = String::with_capacity(code.len());
    visit_node(tree.root_node(), code.as_bytes(), &mut out);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let x = translate_to_starlark(
            &r#"
project(# test
	'wayvnc',
	'c',#test
	version: '0.5.0',
	license: 'ISC',
	default_options: [
		'c_std=gnu11',
	],
)

buildtype = get_option('buildtype')
host_system = host_machine.system()
prefix = get_option('prefix')

c_args = [
	'-DPROJECT_VERSION="@0@"'.format(meson.project_version()),
	'-D_GNU_SOURCE',
]

git = find_program('git', native: true, required: false)
if git.found()
   	git_describe = run_command([git, 'describe', '--tags', '--long'])
   	git_branch = run_command([git, 'rev-parse', '--abbrev-ref', 'HEAD'])
   	if git_describe.returncode() == 0 and git_branch.returncode() == 0
  		c_args += '-DGIT_VERSION="@0@ (@1@)"'.format(
 			git_describe.stdout().strip(),
 			git_branch.stdout().strip(),
  		)
    endif
endif

if buildtype != 'debug' and buildtype != 'debugoptimized'
	c_args += '-DNDEBUG' # comment
endif

add_project_arguments(c_args, language: 'c')

cc = meson.get_compiler('c')

libm = cc.find_library('m', required: false)
librt = cc.find_library('rt', required: false)
libpam = cc.find_library('pam', required: get_option('pam'))

pixman = dependency('pixman-1')
gbm = dependency('gbm', required: get_option('screencopy-dmabuf'))
drm = dependency('libdrm')
xkbcommon = dependency('xkbcommon', version: '>=1.0.0')
wayland_client = dependency('wayland-client')

neatvnc_version = '>=0.5.0'

neatvnc_project = subproject(
	'neatvnc',
	required: false,
	version: neatvnc_version,
)

aml_project = subproject('aml', required: false)
if get_option('neatvncSystem')
    aml = dependency('aml')
else
	aml = aml_project.get_variable('aml_dep')
endif

if get_option('amlSystem')
	neatvnc = dependency('neatvnc', version: neatvnc_version)
else
	neatvnc = neatvnc_project.get_variable('neatvnc_dep')
endif
"#
            .to_string(),
        );
    }
}
