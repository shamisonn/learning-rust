# rust-learning

ref. https://doc.rust-jp.rs/book/second-edition

# 2/8 today I learn

* use clion for learning
* suffix `!` is macro, is not function
* Rustacean: rust coder
* `cargo run` : run, base of cargo.toml
* `cargo check` : is fast check that code can be runnable
* `cargo build` : build executable binary
* rustfmt is cool
* `cargo build --release`: build for release, 
  * compile time is longer than `build` time
  * use for benchmarking?
* cargo ref: https://doc.rust-lang.org/cargo/
  * jp: ??
* `cargo update`: update lock file, by toml
* String function `parse`: need type hinting, because this parse hinted type.
  * and this return `Result`(so we should chain `expect`)
* variable shadowing: same name is allow. if do it, override value.
  * immutable concept is crashed .. ?
    * ref. chapter 3
* `loop` is infinite loop. should use `break`
* edit `.idea/workspace.xml`'s `component name="CargoProjects"` for use multiple cargo projects in clion
  * like this
  
```xml
<component name="CargoProjects">
    <cargoProject FILE="$PROJECT_DIR$/hello_world/Cargo.toml" />
    <cargoProject FILE="$PROJECT_DIR$/variables/Cargo.toml" />
    <cargoProject FILE="$PROJECT_DIR$/guessing_game/Cargo.toml" />
  </component>
```

* 