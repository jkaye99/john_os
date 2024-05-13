# john_os

### Required Dependencies:

<span>
<li>Rust nightly</li>
<li>bootimage</li>
<li>llvm-tools</li>

#### Install Rust via `rustup`:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`


#### In project directory, override the default toolchain:

`rustup toolchain install nightly`

`rustup override set nightly`

#### Install required crates

`cargo install bootimage`

`rustup component add llvm-tools-preview`

After installing `bootimage` and `llvm-tools-preview` run `cargo bootimage` to create a bootable disk image called `bootimage-john_os.bin` in `target/x86_64-john_os/debug`. This can be boot in a virtual machine or copied to a usb drive to boot on real hardware.


#### Running the binary

`cargo run` the will boot the kernel in qemu

#### Running Tests

`cargo test`

</span>
