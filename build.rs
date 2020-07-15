fn main() {
    println!("cargo:rerun-if-changed=src/bsp/qemu-virt/linker.ld");
}
