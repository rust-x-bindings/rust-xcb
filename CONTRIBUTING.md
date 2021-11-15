# Contribution guide

All PRs are welcome.
The source code is organize as follow:

## `build/`

The code generation program (aka. the build script).
This program scans the XML definition to produce the Rust source code.

It is often tedious to understand changes of the Rust generated code only by looking at the code generation source code.
If your PR include a modification of the code generation, please consider including a diff of the generated code.
It is easily done as follow:
 - run `./gen.sh previous` before starting your PR
 - run `./gen.sh current` when you think your work is ready.
 - include the output of `diff -ur gen/previous gen/current` in the text of the PR.
    (see [#110](https://github.com/rust-x-bindings/rust-xcb/pull/110) for an example)

In VsCode you can observe a diff directly in the editor by running `gen/diff.sh [module name]`.

Sometimes, you have to hardcode exceptions in the code generation (e.g. generated different code for a specific request or event).
If you do this (it is better if you don't), add an entry to the `EXCEPTION_LIST.txt` file.

If you need to write unit test for the generated code, you can add them under `src/test.rs`

## `examples/`

Every example is welcome, especially if it covers requests, events or extensions not covered by the current set of examples.
If adding an example, add the corresponding entry in `Cargo.toml`.
The Github workflows ensure that every example compiles.

## `gen/`

Working directory to observe the generated code. The content is not under version control. A sub-directory can be populated by running `./gen.sh [dirname]`.

## `src/`

Base library source code.

## `xml/`

The XML definitions from the XCB project.
These defintions were slightly modified compared to the upstream definitions to ease a part of the code generation.
