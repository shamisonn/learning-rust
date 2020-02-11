# rust-learning

ref. https://doc.rust-jp.rs/book/second-edition

# 2/11 today I learn

* impl todo

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
* String function `parse`: need __type hinting__  type annotation, because this parse __hinted__ annotated type.
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

* `mut` means mutable
* if no `mut`, immutable
* we consider that: 
  * use mutable
    * easy to write, no happen memory allocation
    * for performance tuning, use this, maybe 
    * but code is not clean..
  * use immutable
    * clean code, and no side effect
      * number of bug will be small
    * sometimes, we cant implement algorithm in our brain
* `const` can set only constant formula(= value), no function return
* shadowing is not `mut`
  * **finally**, variable is immutable
  * shadowing can change var type
  * question: is happened memory allocation in every time for using shadowing?
* data type has 2 kinds: scalar type and compound type
* remind: `parse` function need type annotation
* rust has 4 scalar type, integers, floating-point numbers, boolean, characters
* compound types can group multiple values(= scalar type)
* primitive compound types: tuples and arrays
* tuple can have multiple types
* array
  * length is fixed in rust
  * use, when would like to use stack memory than heap
    * details: chapter 4
  * if you like unfixed length array, use `Vector` type
  * if you don't know which choose array or vector, use vector
    * details: chapter 8
* panic: crashed program
  * errors details: chapter 9
* `fn` return last line without `return` word
  * if last line is expression
* statement need `;`
* expression don't have `;`
* if `fn` is empty, return `()`(= empty tuple)
* `if` is expression, so return value, so catch by variable
    * `if` blocks must return same type
* loop is: `while`, `for`, `loop`
  * can use `break`
* `Range` type is useful like: `for num in (1..4)`
 
