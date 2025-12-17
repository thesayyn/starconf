# starconf

Autoconf replacement for Bazel and BCR.

# Goals

- Be fast enough that no one worries about having autoconf in the dependency graph
- Be fully `CCInfo` compatible
- Be powerful enough to express complex `config.h` constraints
- Be fully compatible with CMake-style `config.h.in`
- Play nicely with Platforms and Transitions
- Replace checked-in `config.h` files in BCR

# Try it out

Check out the [examples/simple](./examples/simple) folder for a simple example.

Clone this repo and run `bazel build examples/simple:lib`.

# Problem Statement

The Bazel Central Registry is [full](https://github.com/search?q=repo%3Abazelbuild%2Fbazel-central-registry%20config.h&type=code) of checked-in `config.h` files generated via autoconf. There are multiple `config.h` files per module—for instance, `config_windows.h`, `config_linux_arm64.h`, and so on.

These checked-in `config.h` files assume that the current CC toolchain configuration is roughly the same as the one that generated them. This doesn't work when compiling for an exotic platform or different architecture with a custom CC toolchain configuration.

# Documentation

The Starlark API is strongly typed. Documentation can be found at [docs/lib](./docs/lib/compiler.md).

# FAQ

## What is starconf?

starconf is a program that embeds a Starlark interpreter to express what would be [M4](https://www.gnu.org/software/m4/m4.html)-style conditions in Starlark.

## How does it work?

It works the same way as autoconf. For example, starconf has a `cc.has_header()` function that works like autoconf's `AC_CHECK_HEADER`.

starconf exports a Bazel rule called `starconf` that takes two inputs: a `config.h.in` and a `configure.star` file that describes how to satisfy the conditions inside the `config.h.in` file. Its output is a `config.h` file that can be included in subsequent `cc_library` targets because `starconf` exports `CcInfo`.

starconf can also take `deps` (`cc_library` targets) and make them available in calls such as `has_header_symbol`, `has_header`, and related functions.

## Why not use Bazel's Platforms and Configuration?

Two main reasons:

First, [configuration attributes](https://bazel.build/docs/configurable-attributes) need to be declared statically. That means declaring thousands of flags like `flags//int:have_int16_t` and `flags//int:have_uintmax_t`. Assume these are generated automatically by some tool and there's a canonical module that defines them—that solves part of the problem.

But how does one determine which combination of flags the *current CC toolchain* supports? The answer isn't simple. You'd need to invoke the CC toolchain with some test C code that either builds or fails. Based on that, you'd have to generate a transition that enables some flags while disabling others. And we haven't even gotten to generating a `config.h` to feed into `cc_library` yet. You'd also need something that can *enumerate* the enabled flags and turn them into a `config.h` file. Good luck turning [this libarchive meson.build](https://github.com/mesonbuild/wrapdb/blob/master/subprojects/packagefiles/libarchive/meson.build) into a select statement.

Second, this is a gross oversimplification—there are limitations I haven't mentioned. Bazel doesn't allow you to change the configuration during the execution phase, but that's when we can actually execute the current CC toolchain to determine what it's capable of when compiling against the current target platform. Can't we just do that during the loading phase? No—the "current CC toolchain" means whatever compiler the current target needs, and that isn't available until the analysis phase because of toolchain selection.

## Why Starlark?

Bazel uses Starlark, so it comes naturally to people who work with Bazel frequently.

## How does it compare to autoconf?

It's identical to what autoconf does: inspect the CC toolchain and the current target platform, then generate a `config.h` that allows the C code to compile against the current configuration.
