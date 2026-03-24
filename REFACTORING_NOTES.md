# OreshicRecord リファクタリング検討メモ

## 概要

総行数: 約1,840行 (27ファイル)
最大ファイル: `common.rs` (387行)

---

## 1. common.rs の肥大化 (優先度: 高)

**現状**: 387行に複数の責務が混在

| 責務 | 関数/構造体 |
|------|-------------|
| 型定義 | `Record` |
| Markdownパース | `collect_sections`, `read_plain_block`, `read_code_block` |
| ファイル収集 | `collect_records`, `count_section`, `collect_writeup` |
| 出力 | `print_section`, `print_md`, `convert_image_paths` |
| 型推論 | `infer_record_type` |

**提案**: ファイル分割

```
executor/
├── common/
│   ├── mod.rs          # pub use + Record定義
│   ├── parser.rs       # collect_sections, read_*, collect_writeup
│   ├── collector.rs    # collect_records, count_section
│   └── printer.rs      # print_section, print_md
```

---

## 2. cli.rs の重複定義 (優先度: 中)

**現状**: `SearchCommands` の `Command`, `Track`, `Query` がほぼ同じフラグセット

```rust
// Command, Track, Query すべてに同じパターン
#[arg(long)] run: bool,
#[arg(long)] del: bool,
#[arg(long)] open: bool,
```

**提案**: 共通フラグを抽出

```rust
#[derive(Debug, Args)]
pub struct ActionFlags {
    #[arg(long)]
    pub run: bool,
    #[arg(long)]
    pub del: bool,
    #[arg(long)]
    pub open: bool,
}
```

ただし、clapのflatten/ArgGroupとの相性があるため要検証。

---

## 3. query.rs / command.rs のフラグ処理重複 (優先度: 中)

**現状**: `run`, `del`, `open` の分岐ロジックが両ファイルで類似

```rust
// query.rs:108-139 と command.rs:85-103 がほぼ同じ
if self.run {
    flag::run::exe(section)?;
} else if self.del {
    flag::del::exe(...)?;
} else if self.open {
    flag::open::exe(...)?;
} else {
    common::print_section(section)?;
}
```

**提案**: フラグディスパッチの共通化

```rust
// flag/dispatch.rs
pub fn handle_section_action(
    section: &Record,
    run: bool,
    del: bool,
    open: bool,
) -> Result<()> { ... }
```

---

## 4. Record構造体の複雑さ (優先度: 低)

**現状**: 13フィールド、多くがOption

```rust
pub struct Record {
    pub index: usize,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub path: PathBuf,
    pub unit_type: UnitType,
    pub record_type: RecordType,
    pub count: Option<usize>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub command: Option<String>,
    pub mode: Option<String>,
    pub result: Option<String>,
    pub tags: Vec<String>,
}
```

**提案**: `Default` トレイト実装 or ビルダーパターン

```rust
#[derive(Debug, Clone, Default)]
pub struct Record { ... }

impl Record {
    pub fn new(path: PathBuf, record_type: RecordType) -> Self {
        Self {
            path,
            record_type,
            ..Default::default()
        }
    }
}
```

---

## 5. エラーハンドリング (優先度: 低)

**現状**: `unwrap()` が散在 (特に `pty.rs`)

```rust
// pty.rs:41-44
let mut term = Termios::from_fd(stdin_raw).unwrap();
cfmakeraw(&mut term);
tcsetattr(stdin_raw, TCSANOW, &term).unwrap();
```

**提案**: `?` 演算子 + `anyhow::Context`

```rust
let mut term = Termios::from_fd(stdin_raw)
    .context("failed to get terminal attributes")?;
```

---

## 6. テストの不足 (優先度: 将来)

現状テストファイルが見当たらない。

**提案**: 以下から優先的にテスト追加
- `collect_sections` のパース結果
- `read_code_block` / `read_plain_block`
- `infer_record_type`

---

---

## 7. タイポ・コメント誤り

| ファイル | 行 | 問題 | 修正 |
|----------|-----|------|------|
| `cli.rs` | 146 | `serach` | `search` |
| `record/dispatch.rs` | 62 | `"Rcorded"` | `"Recorded"` |
| `common.rs` | 98 | `line_no + 0; // 1-based` | コメント誤り。`enumerate()`は0-basedなので `// 0-based` が正しい |

### 修正済みバグ: print_section の MD_VIEWER 処理

`print_md` では `viewer.split_whitespace()` で引数を分割していたが、`print_section` では未対応だった。
`MD_VIEWER="mcat --paging never"` のような設定で「No such file or directory」エラーが発生していた。

---

## 8. 冗長なコード

### search/dispatch.rs:47
```rust
executor.run(&ctx)  // &ctx は既に参照なので & 不要
```

### common.rs:98
```rust
let start_line = line_no + 0;  // + 0 は無意味
```

---

## 対応の優先順位

1. **タイポ修正** - 即座に対応可能
2. **common.rs 分割** - 最も効果が高い
3. **フラグ処理の共通化** - 保守性向上
4. **cli.rs の整理** - clap制約があるため慎重に
5. **その他** - 必要に応じて
