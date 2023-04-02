# file_renamer
ファイルの内容に基づき、ファイル名に拡張子を追加、もしくは修正するツールです。

## usage
コマンドラインから以下のように実行してください。
- `file_renamer target/debug/file_renamer [options] 対象ファイル...`
### Options
- `-h, --help`: ヘルプが見れます
- `-d, --dryrun`: ドライランモード(実際の改名は行わないモード)

### Windowsなら...
たぶんSendToとかに登録するのがラクです。あとD&Dとか。

## 対応ファイル
以下のリストを参照ください。
- https://docs.rs/file-format/latest/file_format/enum.FileFormat.html#variants
