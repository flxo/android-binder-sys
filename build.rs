extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("android-binder.h")
        .whitelisted_type("binder_.*")
        .whitelisted_type("transaction_flags.*")
        .whitelisted_type("flat_binder_object")
        .whitelisted_type("BinderDriverCommandProtocol")
        .whitelisted_type("BinderDriverReturnProtocol")
        .whitelisted_type("BinderType")
        .whitelisted_type("Protocol")
        .whitelisted_type("IBinder")
        // TODO: bindgen doesn't support cross generation right now. Figure out how to point
        // bindgen to the cross clang with a proper sysroot
        .clang_arg("--sysroot=/home/felix/audi-hcp/esrlabs/prebuilts/clang/linux-x86_64/eso-arm-linux-androideabi-4.9/sysroot")
        // TODO: This settings here should depend on the target arch
        .clang_arg("-m32")
        // TODO: Add -march etc...
        .layout_tests(false)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
