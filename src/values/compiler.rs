use std::convert::Infallible;
use std::fmt;
use std::fmt::Display;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use allocative::Allocative;
use anyhow::Context;
use starlark;
use starlark::any::ProvidesStaticType;
use starlark::environment::GlobalsBuilder;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::eval::Evaluator;
use starlark::values::starlark_value_as_type::StarlarkValueAsType;
use starlark::values::tuple::UnpackTuple;
use starlark::values::Coerce;
use starlark::values::FreezeResult;
use starlark::values::FrozenValue;
use starlark::values::StarlarkValue;
use starlark::values::StringValue;
use starlark::values::UnpackValue;
use starlark::values::Value;
use starlark::values::ValueLike;
use starlark::{starlark_complex_value, starlark_module};
use starlark_derive::starlark_value;
use starlark_derive::Freeze;
use starlark_derive::NoSerialize;
use starlark_derive::Trace;
use tempfile::NamedTempFile;

#[derive(Clone, Default, Debug, Trace, Freeze, ProvidesStaticType, Allocative, NoSerialize)]
#[repr(C)]
pub struct CompilerGen<'v, V: ValueLike<'v>> {
    pub(crate) name: V::String,
}

impl<'v, V: ValueLike<'v>> Display for CompilerGen<'v, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<compiler>")
    }
}

unsafe impl<'v> Coerce<CompilerGen<'v, Value<'v>>> for CompilerGen<'static, FrozenValue> {}

#[starlark_value(type = "Compiler")]
impl<'v, V: ValueLike<'v>> StarlarkValue<'v> for CompilerGen<'v, V>
where
    Self: ProvidesStaticType<'v>,
{
    fn get_methods() -> Option<&'static Methods> {
        get_compiler_methods()
    }
}

impl<'v> UnpackValue<'v> for Compiler<'v> {
    type Error = Infallible;

    fn unpack_value_impl(value: Value<'v>) -> Result<Option<Self>, Self::Error> {
        // TODO: remove cloned
        Ok(Compiler::from_value(value).cloned())
    }
}

starlark_complex_value!(pub(crate) Compiler<'v>);

impl<'v> Compiler<'v> {
    pub(self) fn _compiles(&self, code: String, args: Option<Vec<String>>) -> bool {
        let mut cmd = Command::new("gcc");
        cmd.arg("-O0") // do not optimize output
            .arg("-v")
            .arg("-c")
            .arg("-x")
            .arg("c++")
            .arg("-") // read the program from stdin
            .arg("-o")
            .arg("/dev/null")
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .stdout(Stdio::null());

        if let Some(args) = args {
            cmd.args(args);
        }

        let cmd = cmd.spawn();
        if cmd.is_err() {
            return false;
        }
        let mut cmd = cmd.unwrap();
        let stdin = cmd.stdin.as_mut().expect("failed to take stdin");
        stdin
            .write_all(code.as_bytes())
            .expect("failed to write to stdin");
        stdin.flush().expect("failed");
        let result = cmd.wait();
        if result.is_err() {
            return false;
        }
        result.unwrap().success()
    }
    pub(self) fn compile_and_run(self, code: String) -> anyhow::Result<Vec<u8>> {
        let tmp = NamedTempFile::new().context("failed to create a temp file")?;
        let mut cmd = Command::new("gcc")
            .arg("-v")
            .arg("-x")
            .arg("c++")
            .arg("-") // read the program from stdin
            .arg("-o")
            .arg(tmp.path())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to spawn the compiler")?;
        let stdin = cmd.stdin.as_mut().context("failed to take stdin")?;
        stdin
            .write_all(code.as_bytes())
            .context("failed to write to stdin")?;
        stdin.flush().context("failed to flush stdin")?;

        let result = cmd.wait_with_output().context("command failed")?;
        if !result.status.success() {
            anyhow::bail!(
                "command exited non-zero code ({})\n{}\n{}",
                result.status.code().unwrap_or(-1),
                String::from_utf8(result.stderr).context("failed to parse compiler output")?,
                String::from_utf8(result.stdout).context("failed to parse compiler output")?
            )
        }

        let cmd = Command::new(tmp.path())
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to spawn the sizeof program")?;
        let output = cmd.wait_with_output().context("program failed")?;
        Ok(output.stdout)
    }
}

pub(super) fn get_compiler_methods() -> Option<&'static Methods> {
    static RES: MethodsStatic = MethodsStatic::new();
    RES.methods(compiler_methods)
}

#[starlark_module]
pub(crate) fn compiler_methods(registry: &mut MethodsBuilder) {
    // https://mesonbuild.com/Reference-manual_returned_compiler.html#compilercompiles
    fn compiles<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] code: StringValue<'v>,
        #[starlark(require = named)] args: Option<Value<'v>>,
        #[starlark(require = named)] dependencies: Option<Value<'v>>,
        #[starlark(require = named)] include_directories: Option<Value<'v>>,
        #[starlark(require = named)] name: Option<StringValue<'v>>,
    ) -> starlark::Result<bool> {
        let compiles = this._compiles(code.to_string(), args.map(|v| vec![v.to_str()]));
        return Ok(compiles);
    }

    fn has_type<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] sym: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
        #[starlark(require = named)] args: Option<Value<'v>>,
    ) -> starlark::Result<bool> {
        let compiles = this._compiles(
            format!(
                r#"
{prefix}
int main(void) {{
    (void) sizeof({});
    return 0;
}}"#,
                sym.to_str(),
                prefix = prefix.map(|f| f.to_str()).unwrap_or("".into())
            ),
            args.map(|v| vec![v.to_str()]),
        );
        return Ok(compiles);
    }

    fn has_header<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] header_name: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
        #[starlark(require = named)] dependencies: Option<Value<'v>>,
    ) -> starlark::Result<bool> {
        let compiles = this._compiles(
            format!(
                r#"{prefix}
#ifdef __has_include
    #if !__has_include("{hname}")
    #error "Header '{hname}' could not be found"
    #endif
#else
    #include <{hname}>
#endif"#,
                hname = header_name.to_str(),
                prefix = prefix.map(|f| f.to_str()).unwrap_or("".into())
            ),
            None,
        );
        return Ok(compiles);
    }

    fn has_header_symbol<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] header_name: Value<'v>,
        #[starlark(require = pos)] symbol: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
    ) -> starlark::Result<bool> {
        let compiles = this._compiles(
            format!(
                r#"{prefix}
#include <{hname}>
int main(void) {{
    /* If it's not defined as a macro, try to use as a symbol */
    #ifndef {symbol}
        {symbol};
    #endif
    return 0;
}}"#,
                hname = header_name.to_str(),
                symbol = symbol.to_str(),
                prefix = prefix.map(|f| f.to_str()).unwrap_or("".into())
            ),
            None,
        );
        return Ok(compiles);
    }

    fn get_supported_arguments<'v>(
        this: Compiler<'v>,
        #[starlark(args)] args: UnpackTuple<Value<'v>>,
    ) -> starlark::Result<Vec<Value<'v>>> {
        // https://github.com/mesonbuild/meson/blob/14010f4dfdb9847944592149b189184ab59b6de0/mesonbuild/compilers/mixins/clike.py#L1295

        let mut working_args = vec![];

        for arg in args.into_iter() {
            if this._compiles(
                r#"extern int i;
                    int i;"#
                    .to_string(),
                Some(vec!["-c".to_string(), arg.to_str()]),
            ) {
                working_args.push(arg);
            }
        }

        return Ok(working_args);
    }

    fn has_member<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] type_name: Value<'v>,
        #[starlark(require = pos)] member_name: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
    ) -> starlark::Result<bool> {
        let compiles = this._compiles(
            format!(
                r#"{prefix}
{prefix}
int main(void) {{
    {typename} foo;
    (void) ( foo.{member} );
    (void) foo;
    return 0;
}}"#,
                typename = type_name.to_str(),
                member = member_name.to_str(),
                prefix = prefix.map(|f| f.to_str()).unwrap_or("".into())
            ),
            None,
        );
        return Ok(compiles);
    }

    fn sizeof<'v>(
        this: Compiler<'v>,
        #[starlark(require = pos)] sym: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
    ) -> anyhow::Result<u32> {
        let output = this.compile_and_run(format!(
            r#"{prefix}
#include<stddef.h>
#include<stdio.h>
int main(void) {{
    printf("%ld", (long)(sizeof({})));
    return 0;
}}"#,
            sym.to_str(),
            prefix = prefix.map(|f| f.to_str()).unwrap_or("".into())
        ))?;

        let output = String::from_utf8(output).context("failed to parse stdout")?;
        output
            .parse::<u32>()
            .context(format!("failed to parse int size, {}", output))
    }

    fn has_function<'v>(
        this: &Compiler<'v>,
        #[starlark(require = pos)] funcname: Value<'v>,
        #[starlark(require = named)] prefix: Option<Value<'v>>,
        #[starlark(require = named)] dependencies: Option<Value<'v>>,
    ) -> anyhow::Result<bool> {
        let prefix = prefix.map(|f| f.to_str()).unwrap_or("".into());

        let func = funcname.to_str();

        // glibc defines functions that are not available on Linux as stubs that
        // fail with ENOSYS (such as e.g. lchmod). In this case we want to fail
        // instead of detecting the stub as a valid symbol.
        // We already included limits.h earlier to ensure that these are defined
        // for stub functions.
        let stubs_fail = r#"
        #if defined __stub_{func} || defined __stub___{func}
        fail fail fail this function is not going to work
        #endif
        "#;

        // If we have any includes in the prefix supplied by the user, assume
        // that the user wants us to use the symbol prototype defined in those
        // includes. If not, then try to do the Autoconf-style check with
        // a dummy prototype definition of our own.
        // This is needed when the linker determines symbol availability from an
        // SDK based on the prototype in the header provided by the SDK.
        // Ignoring this prototype would result in the symbol always being
        // marked as available.

        let mut program = String::new();
        if prefix.contains("#include") {
            // Add the 'prefix', aka defines, includes, etc that the user provides
            // This may include, for instance _GNU_SOURCE which must be defined
            // before limits.h, which includes features.h
            program += format!(
                r#"{prefix}
#include <limits.h>
                "#,
                prefix = prefix
            )
            .as_str();

            program += stubs_fail;

            // We don't know what the function takes or returns, so return it as an int.
            // Just taking the address or comparing it to void is not enough because
            // compilers are smart enough to optimize it away. The resulting binary
            // is not run so we don't care what the return value is.
            program += format!(
                r#"
int main(void) {{
    void *a = (void*) &{func};
    long long b = (long long) a;
    return (int) b;
}}
                "#,
                func = func
            )
            .as_str();
        } else {
            program += format!(
                // Define the symbol to something else since it is defined by the
                // includes or defines listed by the user or by the compiler. This may
                // include, for instance _GNU_SOURCE which must be defined before
                // limits.h, which includes features.h
                // Then, undef the symbol to get rid of it completely.
                r#"
#define {func} meson_disable_define_of_{func}
{prefix}
#include <limits.h>
#undef {func}
        "#,
                prefix = prefix,
                func = func
            )
            .as_str();

            // Override any GCC internal prototype and declare our own definition for
            // the symbol. Use char because that's unlikely to be an actual return
            // value for a function which ensures that we override the definition.
            program += format!(
                r#"
#ifdef __cplusplus
extern "C"
#endif
char {func} (void);
            "#,
                func = func
            )
            .as_str();
            // The actual function call
            program += format!(
                r#"
int main(void) {{
return {func} ();
}}
                "#,
                func = func
            )
            .as_str();
        };

        if this._compiles(program, None) {
            return Ok(true);
        } else {
            let is_builtin = func.starts_with("__builtin_");
            let no_includes = !prefix.contains("#include");
            // Detect function as a built-in
            //
            // Some functions like alloca() are defined as compiler built-ins which
            // are inlined by the compiler and you can't take their address, so we
            // need to look for them differently. On nice compilers like clang, we
            // can just directly use the __has_builtin() macro.
            let compiles = this._compiles(
                format!(
                    r#"{prefix}
            int main(void) {{

            /* With some toolchains (MSYS2/mingw for example) the compiler
             * provides various builtins which are not really implemented and
             * fall back to the stdlib where they aren't provided and fail at
             * build/link time. In case the user provides a header, including
             * the header didn't lead to the function being defined, and the
             * function we are checking isn't a builtin itself we assume the
             * builtin is not functional and we just error out. */
            #if !{no_includes} && !defined({func}) && !{is_builtin}
                #error "No definition for {__builtin_}{func} found in the prefix"
            #endif

            #ifdef __has_builtin
                #if !__has_builtin({__builtin_}{func})
                    #error "{__builtin_}{func} not found"
                #endif
            #elif ! defined({func})
                {__builtin_}{func};
            #endif
            return 0;
            }}"#,
                    func = func,
                    prefix = prefix,
                    __builtin_ = if is_builtin { "" } else { "__builtin_" },
                    is_builtin = std::convert::Into::<i32>::into(is_builtin),
                    no_includes = std::convert::Into::<i32>::into(no_includes),
                ),
                None,
            );
            Ok(compiles)
        }
    }
}

#[starlark_module]
pub fn register_autoconconfig_toplevels(_: &mut GlobalsBuilder) {
    fn get_compiler<'v>(eval: &mut Evaluator<'v, '_, '_>) -> starlark::Result<Compiler<'v>> {
        Ok(Compiler {
            name: eval.heap().alloc_str("clang"),
        })
    }

    const compiler: StarlarkValueAsType<Compiler> = StarlarkValueAsType::new();
}
