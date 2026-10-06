//! Repository automation.
//!
//! `cargo xtask build-fixtures` compiles the firmware fixtures and the
//! vendored fuzzing targets with the ARM bare-metal toolchain. The prebuilt
//! ELFs are committed so the test suite can run without the cross compiler,
//! but this keeps them reproducible.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("build-fixtures") => build_fixtures(),
        Some("build-targets") => build_targets(),
        Some("build-all") => {
            build_fixtures()?;
            build_targets()
        }
        Some("help") | None => {
            println!("usage: cargo xtask <build-fixtures|build-targets|build-all>");
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

    // systick_demo: installs a SysTick handler and halts after three ticks.
    let systick_obj = build.join("systick_demo.o");
    compile(&cc, &common, &src.join("systick_demo.s"), &systick_obj)?;
    let systick_elf = prebuilt.join("systick_demo.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-T")
        .arg(src.join("link.ld"))
        .arg(&systick_obj)
        .arg("-o")
        .arg(&systick_elf);
    run(cmd)?;
    println!("built {}", systick_elf.display());
    Ok(())
}

fn compile(cc: &str, common: &[&str], source: &Path, output: &Path) -> Result<()> {
    let mut cmd = Command::new(cc);
    cmd.args(common).arg("-c").arg(source).arg("-o").arg(output);
    run(cmd)
}

/// Builds the vendored fuzzing targets (currently MQTT-C).
fn build_targets() -> Result<()> {
    let root = repo_root()?;
    let target = root.join("targets").join("mqtt-c");
    let build = root.join("targets").join("build");
    let prebuilt = root.join("targets").join("prebuilt");
    std::fs::create_dir_all(&build)?;
    std::fs::create_dir_all(&prebuilt)?;

    let cc = std::env::var("ARM_GCC").unwrap_or_else(|_| "arm-none-eabi-gcc".to_string());
    let common: [&str; 13] = [
        "-mcpu=cortex-m0",
        "-mthumb",
        "-ffreestanding",
        "-nostdlib",
        "-nostartfiles",
        "-Os",
        "-Wall",
        "-ffunction-sections",
        "-fdata-sections",
        "-I",
        "include",
        "-I",
        "harness",
    ];
    let pal = "-DMQTTC_PAL_FILE=mqtt_pal_min.h";

    // The upstream source lives in targets/mqtt-c; compile from there so the
    // relative include paths resolve.
    let mqtt_obj = build.join("mqtt.o");
    let mut cmd = Command::new(&cc);
    cmd.current_dir(&target)
        .args(common)
        .arg(pal)
        .arg("-c")
        .arg("src/mqtt.c")
        .arg("-o")
        .arg(&mqtt_obj);
    run(cmd)?;

    let compile_harness = |src: &str, out: &str| -> Result<PathBuf> {
        let output = build.join(out);
        let mut cmd = Command::new(&cc);
        cmd.current_dir(&target)
            .args(common)
            .arg(pal)
            .arg("-c")
            .arg(src)
            .arg("-o")
            .arg(&output);
        run(cmd)?;
        Ok(output)
    };

    let main_obj = compile_harness("harness/main.c", "mqtt_main.o")?;
    let compat_obj = compile_harness("harness/compat.c", "mqtt_compat.o")?;
    let startup_obj = compile_harness("harness/startup.s", "mqtt_startup.o")?;

    let elf = prebuilt.join("mqtt_publish.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-Wl,--gc-sections")
        .arg("-T")
        .arg("harness/link.ld")
        .arg(&startup_obj)
        .arg(&main_obj)
        .arg(&compat_obj)
        .arg(&mqtt_obj)
        .arg("-lgcc")
        .arg("-o")
        .arg(&elf)
        .current_dir(&target);
    run(cmd)?;
    println!("built {}", elf.display());

    // Negative control: the same image, but with the CVE-2026-54412 bounds
    // check added to the publish deserialiser. Used to prove that a finding is
    // caused by the missing check and not by an artefact of our harness.
    let source = std::fs::read_to_string(target.join("src/mqtt.c"))?;
    let needle = "response->topic_name_size = __mqtt_unpack_uint16(buf);";
    let guard = format!(
        "{needle}\n    if (response->topic_name_size > mqtt_response->fixed_header.remaining_length - 2) {{\n        return MQTT_ERROR_MALFORMED_RESPONSE;\n    }}"
    );
    if !source.contains(needle) {
        bail!("cannot locate the publish length parse in mqtt.c; upstream changed?");
    }
    std::fs::write(
        build.join("mqtt_fixed.c"),
        source.replacen(needle, &guard, 1),
    )?;

    let fixed_obj = build.join("mqtt_fixed.o");
    let mut cmd = Command::new(&cc);
    cmd.current_dir(&target)
        .args(common)
        .arg(pal)
        .arg("-c")
        .arg("../build/mqtt_fixed.c")
        .arg("-o")
        .arg(&fixed_obj);
    run(cmd)?;

    let fixed_elf = prebuilt.join("mqtt_publish_fixed.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-Wl,--gc-sections")
        .arg("-T")
        .arg("harness/link.ld")
        .arg(&startup_obj)
        .arg(&main_obj)
        .arg(&compat_obj)
        .arg(&fixed_obj)
        .arg("-lgcc")
        .arg("-o")
        .arg(&fixed_elf)
        .current_dir(&target);
    run(cmd)?;
    println!("built {}", fixed_elf.display());

    // Paho MQTTPacket harness.
    let paho = root.join("targets").join("paho");
    let paho_build = paho.join("build");
    std::fs::create_dir_all(&paho_build)?;
    let paho_common: [&str; 14] = [
        "-mcpu=cortex-m0",
        "-mthumb",
        "-ffreestanding",
        "-nostdlib",
        "-nostartfiles",
        "-Os",
        "-Wall",
        "-ffunction-sections",
        "-fdata-sections",
        "-I",
        "mqttpacket",
        "-I",
        "harness",
        "-I",
    ];
    let paho_sources = [
        "mqttpacket/MQTTPacket.c",
        "mqttpacket/MQTTDeserializePublish.c",
        "mqttpacket/MQTTConnectClient.c",
        "mqttpacket/MQTTSubscribeServer.c",
        "mqttpacket/MQTTSubscribeClient.c",
        "mqttpacket/MQTTUnsubscribeServer.c",
        "mqttpacket/MQTTUnsubscribeClient.c",
        "harness/main.c",
        "harness/compat.c",
    ];
    let mut paho_objects = Vec::new();
    for source in paho_sources {
        let stem = source.rsplit('/').next().unwrap().replace(".c", ".o");
        let object = paho_build.join(&stem);
        let mut cmd = Command::new(&cc);
        cmd.current_dir(&paho)
            .args(paho_common)
            .arg("harness/include")
            .arg("-c")
            .arg(source)
            .arg("-o")
            .arg(&object);
        run(cmd)?;
        paho_objects.push(object);
    }
    let paho_startup = paho_build.join("startup.o");
    let mut cmd = Command::new(&cc);
    cmd.current_dir(&paho)
        .arg("-mcpu=cortex-m0")
        .arg("-mthumb")
        .arg("-c")
        .arg("harness/startup.s")
        .arg("-o")
        .arg(&paho_startup);
    run(cmd)?;

    let paho_elf = prebuilt.join("paho_mqtt.elf");
    let mut cmd = Command::new(&cc);
    cmd.args(["-mcpu=cortex-m0", "-mthumb", "-nostdlib", "-nostartfiles"])
        .arg("-Wl,--gc-sections")
        .arg("-T")
        .arg("harness/link.ld")
        .arg(&paho_startup)
        .args(&paho_objects)
        .arg("-lgcc")
        .arg("-o")
        .arg(&paho_elf)
        .current_dir(&paho);
    run(cmd)?;
    println!("built {}", paho_elf.display());
    Ok(())
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
