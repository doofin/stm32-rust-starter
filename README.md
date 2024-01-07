# stm32-rust-starter
template for embeded rust dev with stm32

# setup
```
rustup default nightly
ustup target add thumbv7m-none-eabi
rustup target add thumbv7em-none-eabihf
cargo install cargo-flash
```
# testing
```
cargo build --release
cargo flash --chip stm32f103rb --release

```
# ref
The Embedded Rust Book ： https://docs.rust-embedded.org/book/

Introduction for various peripherals with F3 discovery board: https://docs.rust-embedded.org/discovery/f3discovery/

https://bacelarhenrique.me/2021/02/21/rust-and-stm32-a-quick-start-guide.html

https://medium.com/digitalfrontiers/rust-on-a-stm32-microcontroller-90fac16f6342

https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

forked and simplified from https://gitlab.com/jounathaen-projects/embedded-rust/blinky-rust

Blink the user LED on a Blue pill.

Blog article is https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/
