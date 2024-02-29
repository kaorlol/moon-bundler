# Moon Bundler

Moon Bundler is a simple and easy to use bundler for Lua using the [full-moon](https://github.com/Kampfkarren/full-moon) abstract syntax tree Rust crate.

## Installation

You can install the bundler in [releases](https://github.com/kaorlol/moon-bundler/releases/latest)

#### Custom installation

if you wanna build the executable yourself, you can do the following:

##### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)

##### Building

```sh
git clone https://github.com/kaorlol/moon-bundler.git
cd moon-bundler
cargo build --release
```

## Usage

```sh
moon-bundler -i <input> -o <output>
```

## Todo
- [x] Add CLI implementation
- [x] Add Minify and Beautify options in CLI
- [ ] Add unit tests
- [ ] Refactor, optimize code, and make it more robust
- [ ] Add more support for where acquire is used, i.e. string interpolation
- [ ] Add root file support/option in CLI

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
