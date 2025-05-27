use starlark::environment::GlobalsBuilder;
use starlark::starlark_module; 
use anyhow::anyhow;

#[starlark_module]
pub fn host_machine(_: &mut GlobalsBuilder) {
    fn system<'v>() -> anyhow::Result<&'v str> {
        match std::env::consts::OS {
            "android" => Ok("android"),
            "macos" => Ok("darwin"),
            "ios" => Ok("darwin"),
            "dragonfly" => Ok("dragonfly"),
            "emscripten" => Ok("emscripten"),
            "freebsd" => Ok("freebsd"),
            "hurd" => Ok("gnu"),
            "haiku" => Ok("haiku"),
            "linux" => Ok("linux"),
            "netbsd" => Ok("netbsd"),
            "openbsd" => Ok("openbsd"),
            "windows" => Ok("windows"),
            "solaris" => Ok("sunos"),
            "illumos" => Ok("sunos"),
            _ if cfg!(target_os = "cygwin") => Ok("cygwin"),
            sys => Err(anyhow!("unknown system {}, file an issue.", sys)),
        }
    }

    fn kernel<'v>() -> anyhow::Result<&'v str> {
        match std::env::consts::OS {
            "linux" => Ok("linux"),
            "android" => Ok("linux"), // Android uses Linux kernel
            "freebsd" => Ok("freebsd"),
            "openbsd" => Ok("openbsd"),
            "netbsd" => Ok("netbsd"),
            "hurd" => Ok("gnu"),
            "windows" => Ok("nt"),
            "macos" => Ok("xnu"), // macOS uses XNU kernel
            "ios" => Ok("xnu"),   // iOS uses XNU kernel
            "illumos" => Ok("illumos"),
            "solaris" => Ok("solaris"),
            "dragonfly" =>Ok("dragonfly"),
            "haiku" => Ok("haiku"),
            "emscripten" => Ok("none"), // Emscripten runs in a JavaScript environment, no kernel
            _ if cfg!(target_os = "cygwin") => Ok("nt"), // Cygwin runs on Windows, uses NT kernel
            kernel => Err(anyhow!("unknown kernel {}, file an issue.", kernel)), // Default for unknown or bare-metal systems
        }
    }

    fn subsystem<'v>() -> anyhow::Result<String> {
        match std::env::consts::OS {
            "macos" => Ok("macos".to_string()),
            os @ ("ios" | "tvos" | "visionos" | "watchos") => {
                if cfg!(target_abi = "sim") { 
                    Ok(format!("{os}-simulator"))
                } else { 
                    Ok(os.to_string()) 
                }
            }
            _ => Ok("none".to_string()),
        }
    }

    fn endian<'v>() -> starlark::Result<&'v str> {
        if cfg!(target_endian = "little") {
            Ok("little")
        } else {
            Ok("big")
        }
    }

    fn cpu_family<'v>() -> starlark::Result<&'v str> {
        match std::env::consts::ARCH {
            "aarch64" => Ok("aarch64"),
            "alpha" => Ok("alpha"),
            "arc" => Ok("arc"),
            "arm" => Ok("arm"),
            "avr" => Ok("avr"),
            "csky" => Ok("csky"),
            "ia64" => Ok("ia64"),
            "loongarch64" => Ok("loongarch64"),
            "m68k" => Ok("m68k"),
            "microblaze" => Ok("microblaze"),
            "mips" => Ok("mips"),
            "mips64" => Ok("mips64"),
            "msp430" => Ok("msp430"),
            "parisc" => Ok("parisc"),
            "powerpc" => Ok("ppc"),
            "powerpc64" => Ok("ppc64"),
            "riscv32" => Ok("riscv32"),
            "riscv64" => Ok("riscv64"),
            "rl78" => Ok("rl78"),
            "rx" => Ok("rx"),
            "s390x" => Ok("s390x"),
            "sparc" => Ok("sparc"),
            "sparc64" => Ok("sparc64"),
            "wasm32" => Ok("wasm32"),
            "wasm64" => Ok("wasm64"),
            "x86" => Ok("x86"),
            "x86_64" => Ok("x86_64"),
            _ => Ok("unknown"),
        }   
    }

    fn cpu<'v>() -> anyhow::Result<String> {
        let arch = std::env::consts::ARCH;
        let arch = match arch {
            "x86" => "i686".to_string(),
            "x86_64" => "x86_64".to_string(),
            "arm" => {
                if cfg!(target_feature = "v7") {
                    "armv7l".to_string()
                } else if cfg!(target_feature = "v6") {
                    "armv6l".to_string()
                } else {
                    "arm".to_string()
                }
            }
            "aarch64" => {
                if cfg!(target_feature = "v8.2a") {
                    "armv8.2-a".to_string()
                } else {
                    "armv8-a".to_string()
                }
            }
            "mips" => "mips".to_string(),
            "mips64" => "mips64".to_string(),
            "powerpc" => "ppc".to_string(),
            "powerpc64" => "ppc64".to_string(),
            "riscv32" => "rv32".to_string(),
            "riscv64" => "rv64".to_string(),
            "s390x" => "s390x".to_string(),
            "sparc" => "sparc".to_string(),
            "sparc64" => "sparc64".to_string(),
            _ => "unknown".to_string()
        };
        Ok(arch)
    }
}

pub fn register_toplevels(globals: &mut GlobalsBuilder) {
    globals.namespace("host_machine", |builder| {
        host_machine(builder);
    });
}
