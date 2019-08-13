https://crates.io/crates/fatfs
https://crates.io/crates/arm
https://github.com/gz/rust-multiboot
https://github.com/gz/rust-x86


to build install rust and qume for x86_64


Then we need to install the nightly compiler with:
  -> rustup install nightly
  -> rustup default nightly

  -> rustup component add rust-src
  -> rustup component add llvm-tools-preview
  -> cargo install bootimage
  -> cargo install cargo-xbuild


  -> cargo xbuild --target x86_64-fenrirs_os.json
  -> cargo bootimage --target x86_64-fenrirs_os.json

  -> qemu-system-x86_64 -drive format=raw,file=target/x86_64-fenrirs_os/debug/bootimage-fenrirs_os.bin 