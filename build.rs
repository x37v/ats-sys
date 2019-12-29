use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=atsa");

    let vars = ["ATSA_.*"];
    let types = ["ATS_PEAK", "ATS_FRAME", "ATS_HEADER", "ATS_SOUND", "ANARGS"];
    let funcs = ["main_anal"];

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./ats/ats/src/sndlib")
        .rustfmt_bindings(true);

    builder = vars.iter().fold(builder, |b, i| b.whitelist_var(i));
    builder = funcs.iter().fold(builder, |b, i| b.whitelist_function(i));
    builder = types.iter().fold(builder, |b, i| b.whitelist_type(i));

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .include("ats/ats/src/atsa/")
        .include("ats/ats/src/sndlib/")
        .define("VERSION", "\"1.0.0\"")
        .file("ats/ats/src/sndlib/sound.c")
        .file("ats/ats/src/sndlib/io.c")
        .file("ats/ats/src/sndlib/headers.c")
        .file("ats/ats/src/atsa/atsa.c")
        .file("ats/ats/src/atsa/critical-bands.c")
        .file("ats/ats/src/atsa/other-utils.c")
        .file("ats/ats/src/atsa/peak-detection.c")
        .file("ats/ats/src/atsa/peak-tracking.c")
        .file("ats/ats/src/atsa/residual-analysis.c")
        .file("ats/ats/src/atsa/residual.c")
        .file("ats/ats/src/atsa/save-load-sound.c")
        .file("ats/ats/src/atsa/tracker.c")
        .file("ats/ats/src/atsa/utilities.c")
        .compile("libatsa.a");
}
