fn main() {
    // println!("cargo:rustc-link-search=D:\\Adrien\\Development\\dependencies\\glfw-3.1.2\\build\\src\\Release");
    println!("cargo:rustc-link-search=ext/");
    println!("cargo:rustc-link-lib=user32");
}