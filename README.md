# rusc-v core
a risc-v core in rust.

## prerequisites
- rust
- riscv-gnu-toolchain
- `riscv-testes`

then synthesis in the fpga

## installation
### riscv-tests
```sh
git clone https://github.com/riscv/riscv-tests
cd riscv-tests
git submodule update --init --recursive
autoconf
./configure
make
make install
cd ..
```
getting problems to install riscv-toolchain on MB M1? follow my [recomendations](https://github.com/riscv-collab/riscv-gnu-toolchain/issues/1117#issuecomment-1229446707)

