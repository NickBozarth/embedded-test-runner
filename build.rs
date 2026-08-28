use std::{env, fs, path::PathBuf};

/*
 * The project requires a memory.x file to be linked in order for qemu to run
 */
fn copy_mem_file(out_dir: &PathBuf) {
    let out_mem_file = out_dir.join("memory.x");

    // 64k version is the default, with others potentially in the future
    let memory_x_src = if cfg!(feature = "ram-16k") {
        "memory/memory_16k.x"
    } else {
        "memory/memory_64k.x"
    };

    fs::copy(memory_x_src, out_mem_file).unwrap();

    // Link tell the compiler where the link dir is
    println!("cargo:rustc-link-search={}", out_dir.display());
    // Tell cargo to rerun if the memory source file is changed
    println!("cargo:rerun-if-changed={}", memory_x_src);
}



fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    copy_mem_file(&out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}
