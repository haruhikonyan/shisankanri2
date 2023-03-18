# 開発環境構築
[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/haruhikonyan/shisankanri2)

# wasm(rust)
## 変更検知と自動ビルド
```
$ cd scraping
$ cargo watch -i .gitignore -i "pkg/*" -s "wasm-pack build"
```

# UI(Next.js)
```
$ yarn
$ yarn dev
```
