# 開発環境構築
[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/haruhikonyan/shisankanri2)

# wasm(rust)(TBD)(誰か動くようにして。。。)
## 変更検知と自動ビルド
```
$ cd wasm/scraping
$ cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"
## このままだとビルド通らないので https://github.com/harryfei/which-rs/issues/31#issuecomment-1480631905 を参考に直接 finder と checker を書き換える
```

# UI(Next.js)
```
$ yarn
$ yarn dev
```
