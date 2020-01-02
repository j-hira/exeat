# Exeat
Rustで書かれたコマンドラインツールです.カレントディレクトリから `cd` することなく別ディレクトリにいるかのようにコマンドを実行することができます.

## 実行方法
Cargoが入っている環境で `cargo run <Path> <Command> (<args>..)` でPathのディレクトリにいるかのようにCommandをargsのオプション付きで実行できます.
また `cargo build --release` でバイナリファイルを作成し,パスを通すことで `exeat <Path> <Command> (<args>..)` で実行することができます.(おすすめ)
