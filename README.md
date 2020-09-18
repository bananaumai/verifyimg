# verifyimg

verifyimg is a simple cli tool for verifying the validity of your image file.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## installation

If you already have Go environment on your machine, it's easiest to use `go get`.

```
$ go get github.com/bananaumai/verifyimg
```

Or go to [release page](https://github.com/bananaumai/verifyimg/releases/latest) and download the prebuilt binary which is suitable for your platform.

## usage

```
$ vefifyimg -t (jpeg|png|gif) {FILE_PATH}
```

verifyimg currently supports JPEG, GIF, and PNG image verification.
