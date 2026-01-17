fn main() {
    println!("cargo:rerun-if-changed=components/index.slint");
    println!("cargo:rerun-if-changed=components/buttons.slint");
    println!("cargo:rerun-if-changed=components/meters.slint");
    println!("cargo:rerun-if-changed=components/toggles.slint");
    println!("cargo:rerun-if-changed=components/knobs.slint");
    slint_build::compile("components/index.slint").unwrap();
}