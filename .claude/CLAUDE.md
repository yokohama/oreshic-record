# CLAUDE.md — OreshicRecord 開発者向けリファレンス

## プロジェクト概要

CLIナレッジ管理ツール。バイナリ名は `ors`。
コマンドを実行しながら、その内容・結果をMarkdownとして自動保存し、後から検索・再実行できる。

```
cargo build
cargo install --path .
```

---

## ディレクトリ構成

```
src/
├── main.rs                          # エントリポイント。CLIパースとサブコマンドへのディスパッチ
├── cli.rs                           # clapによるCLI定義（SubCommands, RecordArgs, SearchArgs等）
├── context.rs                       # 環境変数からパスを構築するContext構造体
├── ors_type.rs                      # 共通型（UnitType, RecordType）
├── templates/
│   └── record.md                    # minijinjaテンプレート。記録のMarkdown書式
└── feature/
    ├── mod.rs
    ├── set.rs                       # ors set <track>: .trackファイルに書き込み
    ├── unset.rs                     # ors unset: .trackファイルを削除
    ├── record/
    │   ├── mod.rs
    │   ├── dispatch.rs              # ors record のエントリ。pty/batchを選択してMd.writeを呼ぶ
    │   ├── md.rs                    # Md構造体。minijinjaでレンダリングしてmdファイルに追記
    │   └── executor/
    │       ├── mod.rs               # RecordExecutorトレイト、ExecResult、get_track_name
    │       ├── batch.rs             # 通常コマンド実行（stdout piped、setsid）
    │       └── pty.rs               # PTY実行（interactive用）、ANSIエスケープ除去
    └── search/
        ├── mod.rs
        ├── dispatch.rs              # ors search のエントリ。SearchExecutorを選択
        ├── executor/
        │   ├── mod.rs               # SearchExecutorトレイト
        │   ├── common.rs            # Record構造体、Markdownパース、collect_records/collect_sections
        │   ├── command.rs           # search command / search track
        │   ├── query.rs             # search query（全ディレクトリ横断キーワード検索）
        │   ├── writeup.rs           # search writeup
        │   └── flag/
        │       ├── mod.rs
        │       ├── run.rs           # --run: コマンドを再実行（modeに応じてBatch/Pty）
        │       ├── del.rs           # --del: セクション行範囲を削除 / writeupはファイル削除
        │       └── open.rs          # --open: $EDITORでファイルを開く
        └── table/
            ├── mod.rs               # print_table: ターミナル幅対応・色付きテーブル表示
            ├── md_records.rs        # MDファイル一覧テーブル（Index/Type/Command/Count）
            └── section_records.rs   # セクション一覧テーブル（Index/Type/Title/Command）
```

---

## 主要な型・トレイト

### `Context` (`src/context.rs`)
環境変数 `ORS_RECORDS_DIR` から各ディレクトリパスを導出する。

```rust
pub struct Context {
    pub commands_dir: PathBuf,        // $ORS_RECORDS_DIR/commands/
    pub tracks_dir: PathBuf,          // $ORS_RECORDS_DIR/tracks/
    pub writeups_dir: PathBuf,        // $ORS_RECORDS_DIR/writeups/
    pub track_name_file_path: PathBuf, // $ORS_RECORDS_DIR/tracks/.track
}
```

### `RecordType` / `UnitType` (`src/ors_type.rs`)
- `RecordType`: `Command` / `Track` / `Writeup` — パスの `components()` から推論
- `UnitType`: `MdFile` / `Section` — MDファイル単位かセクション単位か

### `Record` (`src/feature/search/executor/common.rs`)
Markdownを解析した結果の構造体。`collect_records()` で一覧取得、`collect_sections()` でセクション分割。

### `RecordExecutor` トレイト
```rust
pub trait RecordExecutor {
    fn mode(&self) -> &'static str;  // "batch" or "pty"
    fn run(&self) -> anyhow::Result<ExecResult>;
}
```
実装: `Batch`（通常コマンド）、`Pty`（インタラクティブコマンド）

### `SearchExecutor` トレイト
```rust
pub trait SearchExecutor {
    fn run(&self, ctx: &Context) -> anyhow::Result<()>;
}
```
実装: `Command`、`Query`、`Writeup`

---

## Markdownフォーマット

記録はすべて以下のテンプレートで書き込まれる（`src/templates/record.md`）。
1ファイルに複数セクションが追記される形式。`# ` で始まる行がセクション区切り。

```markdown
# <title>

## Message
<message>

## Command
```bash <mode>
<command>
```

## Result
```bash stdout
<result>
```

## Tag
<tags>
```

`mode` は `batch` または `pty`。`--run` 時にこの値でexecutorを選択する。

---

## trackの仕組み

- `ors set <name>` → `tracks/.track` に名前を書き込む
- `ors record` 時に `.track` を読み、存在すれば `tracks/<name>.md` に追記
- `ors unset` → `.track` ファイルを削除
- track未設定時は `commands/<コマンド名>.md` に追記

---

## 検索（search query）のマッチロジック

`query.rs` にて、全3ディレクトリを横断して各セクションに対して以下でマッチ判定：
- `title` に含まれるか（Command/Track）
- `command` に含まれるか（Command/Track）
- `tags` のいずれかに含まれるか（全タイプ）

Writeupはセクション解析せず、ファイル単位でタグのみを検索対象にする。

---

## 関連フォルダ

`/home/banister/projects/oreshic-record.wiki/` がこのツールの実際のデータ格納先（`ORS_RECORDS_DIR`）として機能している。

```
oreshic-record.wiki/
├── commands/          # ors record で蓄積したコマンド記録（apt, nmap, ssh等のMD）
├── tracks/            # ors set <track> で紐付けた記録
├── writeups/
│   ├── md/            # 技術メモ全般（CCNA, Docker, Linux, CTF等）
│   ├── security/      # セキュリティ特化メモ（権限昇格, reverse shell等）
│   ├── thm/           # TryHackMe のルームメモ
│   └── 健康食材レシピ/
├── codes/             # コード断片（C, PHP等）
├── images/            # 画像
└── converter/         # 補助スクリプト（画像変換等）
```

開発時の動作確認には `export ORS_RECORDS_DIR=/home/banister/projects/oreshic-record.wiki` を設定する。

---

## 環境変数

| 変数 | 用途 |
|---|---|
| `ORS_RECORDS_DIR` | 記録ディレクトリのルート（必須） |
| `EDITOR` | `--open` フラグ使用時のエディタ |
| `MD_VIEWER` | セクション表示時のMarkdownビューア（例: `glow`）。未設定時はそのまま標準出力 |

---

## 依存クレート

| クレート | 用途 |
|---|---|
| `clap` | CLIパーサ（derive feature） |
| `minijinja` | Markdownテンプレートレンダリング |
| `anyhow` | エラーハンドリング |
| `portable-pty` | PTY実行 |
| `termios` | rawモード設定 |
| `nix` | poll / signal / waitpid |
| `terminal_size` | テーブル表示幅の取得 |
| `unicode-width` | 全角文字対応のカラム幅計算 |
| `colored` | ターミナルカラー出力 |
| `tempfile` | MDViewerへの一時ファイル渡し |
