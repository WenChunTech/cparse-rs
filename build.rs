use std::path::PathBuf;

fn main() {
    // 编译C++库
    let mut build = cc::Build::new();

    // 设置编译选项
    build
        .cpp(true)
        .opt_level(3) // 添加 -O3 优化
        .include("cparse") // 添加cparse目录到搜索路径
        // 添加异常处理支持
        .flag_if_supported("-fexceptions")
        .flag_if_supported("-frtti")
        // macOS 特定配置
        .flag_if_supported("-arch")
        .flag_if_supported("arm64")
        .flag_if_supported("-stdlib=libc++")
        .flag_if_supported("-std=c++11")
        // 通用配置
        .flag_if_supported("-Wall")
        .flag_if_supported("-pedantic")
        .flag_if_supported("-Wmissing-field-initializers")
        .flag_if_supported("-Wuninitialized")
        .flag_if_supported("-Wsign-compare");

    // macOS 特定链接配置
    if cfg!(target_os = "macos") {
        build.cpp_link_stdlib("c++");
    }

    // 添加源文件 - 分离core源文件和其他源文件
    let core_sources = [
        "cparse/shunting-yard.cpp",
        "cparse/packToken.cpp",
        "cparse/functions.cpp",
        "cparse/containers.cpp",
    ];

    let other_sources = ["cparse/builtin-features.cpp"];

    // 添加所有源文件
    for source in core_sources.iter().chain(other_sources.iter()) {
        build.file(source);
    }

    // 根据操作系统设置特定选项
    if cfg!(target_os = "linux") {
        build.flag("-O1");
    }

    // 编译
    build.compile("cparse");

    // 生成绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .enable_cxx_namespaces()
        // 添加异常处理支持
        .clang_arg("-fexceptions")
        .clang_arg("-frtti")
        // 允许这些类及其方法
        .allowlist_type("cparse::.*")
        .allowlist_function("cparse::.*")
        // 阻止生成可能出现递归的类型
        .blocklist_type(".*__node_pointer")
        .blocklist_type(".*type_")
        .blocklist_type(".*difference_type")
        .blocklist_type(".*node_type")
        // C++ 特性支持
        .opaque_type("std::.*")
        // 派生 trait
        .derive_debug(true)
        .derive_copy(true)
        // 枚举处理
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        // 布局控制
        .size_t_is_usize(true)
        .layout_tests(false)
        // 类型处理
        .translate_enum_integer_types(true)
        .generate_inline_functions(false)
        .rustified_enum(".*")
        .use_core()
        // C++ 配置
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++11")
        .clang_arg("-Icparse")
        // 生成相关函数
        .vtable_generation(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // 修改输出路径到src/bindings
    let out_path = PathBuf::from("src");
    std::fs::create_dir_all(&out_path).expect("Could not create bindings directory");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
