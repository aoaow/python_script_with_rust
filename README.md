# python_script_with_rust
[![CI](https://github.com/aoaow/python_script_with_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/aoaow/python_script_with_rust/actions/workflows/ci.yml)

## Introduction

This project showcases the rewrite of a Python data processing script in Rust, highlighting improvements in speed and resource usage.

## Features

- A python script to read dataset and transform one of its column
- A rust application to show case the improvements compared to the script
  
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

Rust signifantly reduced the time and resource used for data processing via the time command.


| Language | Execution Time (s) | Memory Usage (MB) |
|----------|--------------------|-------------------|
| Python   | 0.137            | 150               |
| Rust     | 0.03              | 50                |
