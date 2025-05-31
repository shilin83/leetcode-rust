## LeetCode-Rust

[![codecov](https://codecov.io/gh/shilin83/leetcode-rust/graph/badge.svg?token=9QBJ5DH10C)](https://codecov.io/gh/shilin83/leetcode-rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/shilin83/leetcode-rust/ci.yml?branch=main&logo=github&label=CI)

## 创建应用

```shell
git clone https://github.com/shilin83/leetcode-rust.git
```

## 安装依赖

```shell
cargo install cargo-llvm-cov cargo-nextest

rustup component add llvm-tools-preview
```

## 运行测试

```shell
cargo nextest run
```

## 覆盖报告

```shell
cargo llvm-cov nextest --open --ignore-filename-regex lib.rs
```

## 题库列表

| 序号                              | 题目                                              | 标签  | 难度 |
|:--------------------------------|:------------------------------------------------|:---:|:--:|
| [0001](src/problems/two_sum.rs) | [✅ 两数之和](https://leetcode.cn/problems/two-sum/) | 哈希表 | 简单 |
