fn main() {
    cc::Build::new()
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c")
        .flag("-fPIC")
        .compile("openglwrapper");


    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
}
