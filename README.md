# Exeat
Rustで書かれたコマンドラインツールです.カレントディレクトリから `cd` することなく別ディレクトリにいるかのようにコマンドを実行することができます.

## 実行方法
Cargoが入っている環境でこのコマンドでPathのディレクトリにいるかのようにCommandをargsのオプション付きで実行できます.
```bash
cargo run <Path> <Command> (<args>..)
```
またこのコマンドでバイナリファイルを作成してパスを通すことで
```bash
cargo build --release
```
このように実行できるようになるのでおすすめです.
```bash
exeat <Path> <Command> (<args>..)
```
