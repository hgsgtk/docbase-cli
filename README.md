# Outline
This is a command line interface application which handle [DocBase](https://docbase.io/) api.

## What is DocBase
[DocBase](https://docbase.io/) is a document sharing service for team.  

- [API Document](https://help.docbase.io/posts/45703)

## Basic Design
### Interface
#### API can use this features.
- メモの検索
- 所属チーム取得
- 所属グループ取得
- タグの取得
- メモの投稿
- メモの詳細取得
- メモの更新
- メモの削除
- コメント投稿
- コメント削除
- ファイルアップロード

#### Command Line Interface
##### Phase-1
- post (メモの投稿）
- config （自分のアクセスキーの登録）

#### ToDo
- [x] --helpでUSAGEを出力する
- [x] APIクライアントの作成（hyper）
- [ ] docoptからapiを呼び出す

- [ ] 複数ドメインに所属している場合の挙動
- [ ] ローカルのmdファイルをpostに投げる:


# References
- [Rust のコマンドラインオプション解析色々](http://ubnt-intrepid.hatenablog.com/entry/rust_commandline_parsers)
