use std::env;
use std::path::PathBuf;

use libbpf_cargo::SkeletonBuilder;

const SRC: &str = "bpf/syslantern.bpf.c";

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR must be set"))
        .join("syslantern.skel.rs");

    SkeletonBuilder::new()
        .source(SRC)
        .build_and_generate(&out)
        .expect("failed to build BPF skeleton");

    println!("cargo:rerun-if-changed=bpf");
}