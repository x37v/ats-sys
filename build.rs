use std::env;
use std::path::PathBuf;

fn main() {
    let vars = ["ATSA_CRITICAL_BANDS"];
    let types = ["ATS_PEAK", "ATS_FRAME", "ATS_HEADER"];

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./sndlib")
        .rustfmt_bindings(true);

    builder = vars.iter().fold(builder, |b, i| b.whitelist_var(i));
    //builder = funcs.iter().fold(builder, |b, i| b.whitelist_function(i));
    builder = types.iter().fold(builder, |b, i| b.whitelist_type(i));

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
