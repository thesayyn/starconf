load("@rules_cc//cc:defs.bzl", "CcInfo", "cc_common")

def _autoconf_impl(ctx):
    ctx.actions.run(
        inputs = [ctx.file.config_in],
        outputs = [ctx.outputs.config_out],
        arguments = ["--output", ctx.outputs.config_out.path],
        executable = ctx.executable._starconf,
    )
    return CcInfo(
        compilation_context = cc_common.create_compilation_context(
            headers = depset([ctx.outputs.config_out])
        )
    )

autoconf = rule(
    implementation = _autoconf_impl,
    attrs = {
        "starlark_config": attr.label(mandatory = True, allow_single_file = True),
        "config_in": attr.label(mandatory = True, allow_single_file = True),
        "config_out": attr.output(mandatory = True),
        "_starconf": attr.label(
            default = "@starconf//:starconf",
            executable = True,
            cfg = "exec"
        ),
    },
)
