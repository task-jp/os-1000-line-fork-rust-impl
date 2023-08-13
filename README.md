参考サイト
https://operating-system-in-1000-lines.vercel.app/ja/welcome

Rust の関数がみれる objdump

```
 cargo-objdump kernel_elf --release -- -d
```

各関数や変数のアドレス

```
cargo-nm kernel_elf --release
```
