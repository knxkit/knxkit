# Datapoints Generator for [knxkit](https://crates.io/crates/knxkit)

Extracts `knx_master.xml` from a project file and generates generic and specific data structures. Currently, only the Rust language is supported.

Normally, you should not use this utility directly. Pre-generated rust structures are available in the [knxkit_dpt](https://crates.io/crates/knxkit_dpt) crate.

##### Usage
```console
dptgen --project=MyProject.knxproj --destination=~/my_project/src/generated --language=rust
```