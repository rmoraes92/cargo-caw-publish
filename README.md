# cargo-caw-publish (checksum aware wrapper publish)
A thin wrapper around `cargo publish` that verifies if a crate is
publishable taking on account both version string and checksum.


## Install

`cargo install cargo-caw-publish`


## Usage

- verify the Cargo.toml file at the root of the current folder.

```bash
$ cargo caw-publish
```

- verify the Cargo.toml file at a path `<package_name>/Cargo.toml`

```bash
$ cargo caw-publish <package_name>
```

- pass extra arguments for the "cargo package" phase

```bash
$ cargo caw-publish --package-args="--allow-dirty --keep-going"
```

- pass extra arguments for the "cargo publish" phase

```bash
$ cargo caw-publish --publish-args="--all-features --keep-going"
```

## Outputs

Let's say our latest release was the version 1.2.3. So:

- when you try to re-run publish with the same version we will return
(exit code 0):

```
the version <ver_string> is already published at the remote registry. nothing
to do here.
```

- when you change some code and forgets to update the version string we will
return (exit code 1):

```
the version <ver_string> is already published at the remote registry but your
local .crate checksum differs from the one on the remote:
local : <hash-a>
remote: <hash-b>
hint: maybe you forgot to update the version at <ver_string>!?
```


## License

The MIT License (MIT)

Copyright © 2024 Ramon Moraes <ramonmoraes.foss@gmail.com>

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
