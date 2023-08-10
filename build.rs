fn main() {
    println!("cargo:rustc-cfg=model_size=\"13B\"");
    if std::is_x86_feature_detected!("avx512f") {
        println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=+avx,+avx2,+fma,+sse3,+avx512f target-cpu=native");
    }
    else if std::is_x86_feature_detected!("avx2") {
        println!("cargo:rustc-env=RUSTFLAGS=-C target-feature=+avx,+avx2,+fma,+sse3 target-cpu=native");
    }
}