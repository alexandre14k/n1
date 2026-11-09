// src/link/build.rs

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

const BUILD_FLASH_ORIGIN: &str = "0x00000000";
const BUILD_RAM_ORIGIN: &str = "0x10000000";
const BUILD_VECTOR_SIZE: &str = "0xC0";
const BUILD_RAM_RESERVED: &str = "32";
const BUILD_STACK_MIN: &str = "0x200";

fn build_feature_name_get(name: &str) -> String {
    format!("CARGO_FEATURE_{}", name.to_uppercase())
}

fn build_feature_active(name: &str) -> bool {
    env::var(build_feature_name_get(name)).is_ok()
}

fn build_family_require() {
    if !build_feature_active("lpc111x") {
        panic!("missing lpc111x family feature");
    }
}

fn build_flash_size_get() -> &'static str {
    if build_feature_active("lpc1115") {
        "64K"
    } else if build_feature_active("lpc1114") {
        "32K"
    } else {
        panic!("unsupported LPC111x device");
    }
}

fn build_ram_size_get() -> &'static str {
    if build_feature_active("lpc1115") {
        "8K"
    } else if build_feature_active("lpc1114") {
        "8K"
    } else {
        panic!("unsupported LPC111x device");
    }
}

fn build_lines_join(lines: &[&str]) -> String {
    let mut text = String::new();
    for line in lines {
        text.push_str(line);
        text.push('\n');
    }
    text
}

fn build_script_memory_get(flash: &str, ram: &str) -> String {
    let flash_line = format!(
        "  FLASH (rx) : ORIGIN = {}, LENGTH = {}",
        BUILD_FLASH_ORIGIN, flash
    );
    let ram_line = format!(
        "  RAM  (rwx) : ORIGIN = {}, LENGTH = {}",
        BUILD_RAM_ORIGIN, ram
    );
    build_lines_join(&[
        "MEMORY",
        "{",
        &flash_line,
        &ram_line,
        "}",
        "",
    ])
}

fn build_script_entry_get() -> String {
    build_lines_join(&["ENTRY(hal_boot_reset)", ""])
}

fn build_script_vector_get() -> String {
    let check = format!(
        "  ASSERT(SIZEOF(.isr_vector) == {}, \"vector size\")",
        BUILD_VECTOR_SIZE
    );
    build_lines_join(&[
        "  .isr_vector ORIGIN(FLASH) :",
        "  {",
        "    KEEP(*(.isr_vector))",
        "  } > FLASH",
        &check,
        "",
    ])
}

fn build_script_text_get() -> String {
    build_lines_join(&[
        "  .text :",
        "  {",
        "    . = ALIGN(4);",
        "    *(.text*)",
        "    . = ALIGN(4);",
        "  } > FLASH",
        "",
    ])
}

fn build_script_rodata_get() -> String {
    build_lines_join(&[
        "  .rodata :",
        "  {",
        "    . = ALIGN(4);",
        "    *(.rodata*)",
        "    . = ALIGN(4);",
        "  } > FLASH",
        "",
    ])
}

fn build_script_data_get() -> String {
    build_lines_join(&[
        "  .data :",
        "  {",
        "    . = ALIGN(4);",
        "    _sdata = .;",
        "    *(.data*)",
        "    . = ALIGN(4);",
        "    _edata = .;",
        "  } > RAM AT> FLASH",
        "  _sidata = LOADADDR(.data);",
        "",
    ])
}

fn build_script_bss_get() -> String {
    build_lines_join(&[
        "  .bss (NOLOAD) :",
        "  {",
        "    . = ALIGN(4);",
        "    _sbss = .;",
        "    *(.bss*)",
        "    *(COMMON)",
        "    . = ALIGN(4);",
        "    _ebss = .;",
        "  } > RAM",
        "",
    ])
}

fn build_script_stack_get() -> String {
    let top = format!(
        "  _estack = ORIGIN(RAM) + LENGTH(RAM) - {};",
        BUILD_RAM_RESERVED
    );
    let check = format!(
        "  ASSERT(_ebss + {} <= _estack, \"ram overflow\")",
        BUILD_STACK_MIN
    );
    build_lines_join(&[&top, &check, ""])
}

fn build_script_discard_get() -> String {
    build_lines_join(&[
        "  /DISCARD/ :",
        "  {",
        "    *(.ARM.exidx*)",
        "    *(.ARM.extab*)",
        "  }",
    ])
}

fn build_script_sections_get() -> String {
    let mut text = String::new();
    text.push_str("SECTIONS\n{\n");
    text.push_str(&build_script_vector_get());
    text.push_str(&build_script_text_get());
    text.push_str(&build_script_rodata_get());
    text.push_str(&build_script_data_get());
    text.push_str(&build_script_bss_get());
    text.push_str(&build_script_stack_get());
    text.push_str(&build_script_discard_get());
    text.push_str("}\n");
    text
}

fn build_script_content_get(flash: &str, ram: &str) -> String {
    let mut text = String::new();
    text.push_str(&build_script_memory_get(flash, ram));
    text.push_str(&build_script_entry_get());
    text.push_str(&build_script_sections_get());
    text
}

fn build_file_write(path: &PathBuf, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

fn build_link_args_emit(out_dir: &str) {
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-arg=-Tlink.ld");
    println!("cargo:rerun-if-changed=src/link/build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_LPC111X");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_LPC1114");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_LPC1115");
}

fn main() {
    build_family_require();
    let out_dir = env::var("OUT_DIR").unwrap();
    let flash = build_flash_size_get();
    let ram = build_ram_size_get();
    let content = build_script_content_get(flash, ram);
    let dest_path = PathBuf::from(&out_dir).join("link.ld");
    build_file_write(&dest_path, &content);
    build_link_args_emit(&out_dir);
}