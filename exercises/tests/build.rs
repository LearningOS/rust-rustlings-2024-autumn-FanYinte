fn main() {
    // 在 tests7 中，我们应该设置一个环境变量 `TEST_FOO`
    // 这个时间戳用来确保我们在测试中可以比较当前时间。
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，我们需要启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
