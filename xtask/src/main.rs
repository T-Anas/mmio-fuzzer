//! Repository automation.
//!
//! `cargo xtask build-fixtures` compiles the firmware fixtures with the ARM
//! bare-metal toolchain. The prebuilt ELFs are committed so the test suite can
//! run without the cross compiler, but this keeps them reproducible.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("build-fixtures") => build_fixtures(),
        Some("help") | None => {
            println!("usage: cargo xtask build-fixtures");
            Ok(())
        }
        Some(other) => bail!("unknown xtask `{other}`"),
    }
}

fn repo_root() -> Result<PathBuf> {
    // CARGO_MANIFEST_DIR points at xtask/; its parent is the repository root.
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .context("xtask has no parent directory")
}

fn build_fixtures() -> Result<()> {
    let root = repo_root()?;
    let fixture = root.join("fixtures");
    let src = fixture.join("src");
    let build = fixture.join("build");
    let prebuilt = fixture.join("prebuilt");
    std::fs::create_dir_all(&build)?;
    std::fs::create_dir_all(&prebuilt)?;

    let cc = std::env::var("ARM_GCC").unwrap_or_else(|_| "arm-none-eabi-gcc".to_string());
    let common: [&str; 7] = [
        "-mcpu=cortex-m0",
        "-mthumb",
        "-ffreestanding",
        "-nostdlib",
        "-nostartfiles",
        "-Os",
        "-Wall",
    ];

    let startup_obj = build.join("startup.o");
    compile(&cc, &common, &src.join("startup.s"), &startup_obj)?;
    let main_obj = build.join("uart_demo.o");
    compile(&cc, &common, &src.join("uart_demo.c"), &main_obj)?;

    let elf = prebuilt.join("uart_demo.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-T")
        .arg(src.join("link.ld"))
        .arg(&startup_obj)
        .arg(&main_obj)
        .arg("-o")
        .arg(&elf);
    run(cmd)?;
    println!("built {}", elf.display());

    // guard_demo: a tiny assembly-only firmware that reads past its buffer.
    let guard_obj = build.join("guard_demo.o");
    compile(&cc, &common, &src.join("guard_demo.s"), &guard_obj)?;
    let guard_elf = prebuilt.join("guard_demo.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-T")
        .arg(src.join("link.ld"))
        .arg(&guard_obj)
        .arg("-o")
        .arg(&guard_elf);
    run(cmd)?;
    println!("built {}", guard_elf.display());
    Ok(())
}

fn compile(cc: &str, common: &[&str], source: &Path, output: &Path) -> Result<()> {
    let mut cmd = Command::new(cc);
    cmd.args(common).arg("-c").arg(source).arg("-o").arg(output);
    run(cmd)
}

fn run(mut cmd: Command) -> Result<()> {
    let status = cmd.status().with_context(|| {
        format!(
            "failed to spawn {:?}; is arm-none-eabi-gcc on PATH?",
            cmd.get_program()
        )
    })?;
    if !status.success() {
        bail!("command failed: {cmd:?}");
    }
    Ok(())
}
