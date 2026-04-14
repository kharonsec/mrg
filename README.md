# mrg - Interactive Three-Way Merge

`mrg` is an interactive TUI tool for resolving git conflict markers. It detects conflict markers in a file and provides a simple interface to navigate and resolve them by picking sides or editing manually.

## Installation

### One-liner (requires Rust/Cargo)
```bash
curl -fsSL https://raw.githubusercontent.com/kharonsec/mrg/master/install.sh | bash
```

### Manual
```bash
git clone https://github.com/kharonsec/mrg.git
cd mrg
./install.sh
```

## Usage

```bash
mrg <file_with_conflicts>
```

### TUI Controls
- **Arrow Keys**: Navigate between conflict hunks.
- **1**: Pick the "Ours" side.
- **2**: Pick the "Theirs" side.
- **3**: Pick the "Base" side (if available).
- **s**: Save the resolved file and exit.
- **q**: Quit without saving.
