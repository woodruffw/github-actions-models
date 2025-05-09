github-actions-models
=====================

[![CI](https://github.com/zizmorcore/github-actions-models/actions/workflows/ci.yml/badge.svg)](https://github.com/zizmorcore/github-actions-models/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/github-actions-models)](https://crates.io/crates/github-actions-models)

Unofficial, high-quality data models for GitHub Actions workflows, actions, and related components.

## Why?

I need these for [another tool], and generating them automatically from
[their JSON Schemas] wasn't working both for expressiveness and tool deficiency
reasons.

[another tool]: https://github.com/woodruffw/zizmor

[their JSON Schemas]: https://www.schemastore.org/json/

## License

MIT License.

The integration tests for this crate contain sample workflows collected from
various GitHub repositories; these contain comments linking them to their
original repositories and are licensed under the terms there.
