# cargo-caw-publish (checksum aware wrapper publish)
A thin wrapper around `cargo publish` that verifies if a crate is
publishable taking on account both version string and checksum.


## Install

`cargo install cargo-caw-publish`


## Usage

- if you don't a crate/package name it will be assumed you want to process
the Cargo.toml file at the root of the current folder.

```bash
$ cargo caw-publish
```

- if passing crate/package name it will be assumed you want to process
a Cargo.toml file at <package_name>/Cargo.toml

```bash
$ cargo caw-publish <package_name>
```

- if you need to pass extra arguments for the "cargo package" phase

```bash
$ cargo caw-publish --package-args="--allow-dirty --keep-going"
```

- if you need to pass extra arguments for the "cargo publish" phase

```bash
$ cargo caw-publish --publish-args="--all-features --keep-going"
```


## License

The MIT License (MIT)

Copyright © 2024 Ramon Moraes

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
