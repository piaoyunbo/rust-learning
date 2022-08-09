# 概要

AWS の利用料金を slack のチャンネルに通知するコマンドラインツールを rust で作ったサンプルです。

# 事前準備

- AWS の Cost Explorer を有効にする
- .envファイル作成
```
AWS_REGION=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
AWS_SESSION_TOKEN=
SLACK_WEBHOOK_URL=
```

# 実行

```shell
cargo run -- aws-cost
```