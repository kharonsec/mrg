# mrg

Interactive three-way merge tool. Detects conflict markers and opens an interactive TUI to resolve them.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
mrg <file>
```

### Controls:
- **Arrow Keys**: Navigate between hunks.
- **1**: Pick "Ours" side.
- **2**: Pick "Theirs" side.
- **3**: Pick "Base" side (if available).
- **s**: Save and exit.
- **q**: Quit without saving.
