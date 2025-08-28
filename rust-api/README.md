## Installation
0. **npm/Node.js**
    - [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

1. **Rust**
    - [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) must add the C++ builder
    - [Rust](https://www.rust-lang.org/tools/install) itself



## Commands
- `cargo run` runs the `main.rs` in the current directory
- `cargo add/remove {dependency}`
- `cargo add {dependency} --features {feature1},{feature2}`

- `docker-compose down -v` - take down the container AND the PGSQL setup
- `docker-compose up` - startup the container(s)