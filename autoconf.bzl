def _autoconf_impl(ctx):
    ctx.actions.run(
        inputs = [ctx.file.config_in],
        outputs = [ctx.outputs.config_out],
        arguments = ["--output", ctx.outputs.config_out.path],
        executable = ctx.executable._autoconf,
    )

autoconf = rule(
    implementation = _autoconf_impl,
    attrs = {
        "config_in": attr.label(mandatory = True, allow_single_file = True),
        "config_out": attr.output(mandatory = True),
        "_autoconf": attr.label(mandatory = True, executable = True, cfg = "exec"),
    },
    executable = True,
)
