# python_script_with_rust
[![CI](https://github.com/aoaow/python_script_with_rust/actions/workflows/main.yml/badge.svg)](https://github.com/aoaow/python_script_with_rust/actions/workflows/ci.yml)

## Introduction

This project showcases the rewrite of a Python data processing script in Rust, highlighting improvements in speed and resource usage.

## Installation

### Python Script

```bash
pip install -r requirements.txt
```

### Rust Script

```bash
rustup install stable
cargo build --release
```

## Usage

```bash
cargo run --release
```

## Performance Comparison

Rust signifantly reduced the time and resource used for data processing.


| Language | Execution Time (s) | Memory Usage (MB) |
|----------|--------------------|-------------------|
| Python   | 0.137            | 150               |
| Rust     | 0.03              | 50                |
