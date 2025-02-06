load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    integrity = "sha256-4qOr06EWg6BTBbv9M8xJTfebNkE9VMBXsa17RZD071I=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.57.1/rules_rust-0.57.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

# 使用本地安装的Rust工具链
rust_repositories(
    rust_toolchain = "@local_config_rust//:toolchain",
    register_default_toolchain = False,  # 不注册默认工具链
)

# 加载本地Rust工具链配置
load("@local_config_rust//:toolchain.bzl", "rust_toolchain")

# 注册本地Rust工具链
rust_toolchain(
    name = "local_rust_toolchain",
    toolchain_path = "$HOME/.cargo",  # 替换为实际的本地Rust工具链路径
    is_default = True,
)
