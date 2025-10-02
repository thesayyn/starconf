# starconf

autoconf replacement for Bazel and BCR.

# Problem Statement

Bazel central registry is [full](https://github.com/search?q=repo%3Abazelbuild%2Fbazel-central-registry%20config.h&type=code) of checked in `config.h` files generated via autoconf. there are multiple `config.h` per module, for instance `config_windows.h` `config_linux_arm64.h` and so on.

These checked in `config.h` files assume that the current CC toolchain configuration somewhat same as the one that generated the config.h file, this does not work if you are compiling against some exotic platform or different architecture with a custom CC toolchain configuration.

# Documentation

Starlark API is strongly typed, and its documentation can be found at [docs/lib](./docs/lib/compiler.md)

# FAQ


## what is starconf

starconf is a program that embeds a starlark interpreter to allow describing what would be [M4](https://www.gnu.org/software/m4/m4.html) style conditions in starlark.

## how does it work

It works the same way as autoconf, for example starconf has a `cc.has_header()` function that works the same way as autoconfs `AC_CHECK_HEADER`.

starconf exports a bazel rule `starconf` that takes 2 things as inputs; a `config.h.in` and a `configure.star` file that describes how to satisfy the conditions inside of `config.h.in` file. its output is a `config.h` file that can be included in the subsequent `cc_library` targets because `starconf` is a rule that exports `CcInfo`.
starconf can also take `deps` (cc_library) targets and make them available in calls such as `has_header_symbol`, `has_header` and its variants.


## why not use Bazels Platforms and Configuration

Two main reasons;

[configuration attributes](https://bazel.build/docs/configurable-attributes) needs to be declared statically first, that means declaring thousands of flags, eg `flags//int:have_int16_t`, `flags//int:have_uintmax_t`.
assume these are generated automagically by some tool and there is canonical module that defines them. that solves the problem.

now, how does one determine what combination of these flags that the *current CC toolchain* supports? answer is not an easy one. one would need to invoke the CC toolchain with some magic `C` code that either builds or not. based on that the we'd have to generate a transition that enables some flags while disabling others. so far we haven't gotten to the part where we generate a config.h to feed into the cc_library. now we need something that can *enumerate* the enabled flags and turn that into config.h file to take part in the compilation. good luck turning [this](https://github.com/mesonbuild/wrapdb/blob/master/subprojects/packagefiles/libarchive/meson.build) into a select statement.

this is a gross oversimplification of the problem, there are some limitations that i have not mentioned yet. Bazel does not allow you to change the configuration of the during execution phase, but that's when we can actually execute the current `CC` toolchain to determine what its capable of when compiling against the current `target platform`. Can't we just do that during loading phase then? NO. remember that current CC toolchain means whatever current target needs as a compiler and that is not avaiable up until analysis phase because of toolchain selection.


## why starlark?

Bazel uses starlark, so comes naturally to people who engage with Bazel frequently.


## how does it compare to autoconf?

Its identical to what autoconf does; inspect the `CC` toolchain and the current `target platform` and generate a config.h that allows the C code to compile against the current configuration.
