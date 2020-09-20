# verifyimg

verifyimg is a simple cli tool for verifying the validity of your image file.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## installation

If you already have Rust environment on your machine, it's easiest to use `cargo`.

```
$ cargo install verifyimg
```

Or go to [release page](https://github.com/bananaumai/verifyimg/releases/latest) and download the prebuilt binary which is suitable for your platform.

## usage

```
$ vefifyimg -f <(jpeg|png|gif)> <FILE>
```

verifyimg currently supports JPEG, GIF, and PNG image verification.
