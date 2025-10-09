load("@rules_cc//cc:defs.bzl", "CcInfo", "cc_common")
load("@rules_cc//cc:find_cc_toolchain.bzl", "CC_TOOLCHAIN_ATTRS", "CC_TOOLCHAIN_TYPE", "use_cc_toolchain", "find_cpp_toolchain")

def _autoconf_impl(ctx):
    out = ctx.outputs.config_out or ctx.actions.declare_file("_%s/config.h" % ctx.label.name)

    toolchain = find_cpp_toolchain(ctx)
    args = ctx.actions.args()
    args.add(out, format="--output=%s")
    args.add(ctx.file.config_in, format="--input=%s")
    args.add(ctx.file.starlark_config, format="--config=%s")
    args.add(toolchain.compiler_executable, format="--cc=%s")
    args.add(toolchain.compiler, format="--compiler=%s")
    args.add_all(toolchain.built_in_include_directories, format_each = "--isystem=%s")
    ctx.actions.run(
        inputs = depset([ctx.file.config_in, ctx.file.starlark_config], transitive = [toolchain.all_files]),
        outputs = [out],
        arguments = [args],
        executable = ctx.executable._starconf,
        toolchain = CC_TOOLCHAIN_TYPE,
        mnemonic = "autoconf",
        progress_message = "autoconf %{label}"
    )

    includes = None
    if ctx.attr.include:
        includes = depset([out.dirname])

    return CcInfo(
        compilation_context = cc_common.create_compilation_context(
            headers = depset([out]),
            includes = includes,
            quote_includes = includes
        )
    )

autoconf = rule(
    implementation = _autoconf_impl,
    attrs = {
        "starlark_config": attr.label(mandatory = True, allow_single_file = True),
        "config_in": attr.label(mandatory = True, allow_single_file = True),
        "config_out": attr.output(),
        "include": attr.bool(default = True, doc = """\
Whether to add config basename directory as include path to allow importing
of config.h as `<config.h>` or `"config.h"` througout the repository.
"""),
        "_starconf": attr.label(
            default = "@starconf//:starconf",
            executable = True,
            cfg = "exec"
        ),
    } | CC_TOOLCHAIN_ATTRS,
    toolchains = use_cc_toolchain()
)
