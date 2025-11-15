# 🔥 Advanced Code Editor Pro

A professional, feature-rich code editor built with **Rust** and **egui**, inspired by **VS Code** and **Lapce**. This is a desktop application that provides a complete integrated development environment with file management, syntax support, and an interactive terminal.

---

## ✨ Features

### 📝 Code Editing
- **Syntax-aware editor** with monospaced font for better readability
- **Line numbering** for easy code navigation
- **Auto-bracket completion** - Automatically closes brackets:
  - `(` → `()`
  - `[` → `[]`
  - `{` → `{}`
  - `"` → `""`
  - `'` → `''`
- **Multi-file editing** with tab support
- **Auto-save** functionality
- Beautiful **Dracula-inspired color scheme**

### 📂 File Management
- **Folder browser** with recursive file tree view
- **File explorer sidebar** showing:
  - Hierarchical folder structure
  - File icons based on file type
  - Proper indentation for nested folders
- **Breadcrumb navigation** for easy path traversal
- **Tab system** with close buttons (✕)
- **Quick file opening** from file dialog

### 🖥️ Terminal Integration
- **Interactive terminal** built-in at the bottom of the editor
- **PowerShell support** - Launch PowerShell directly
- **CMD support** - Launch Windows CMD
- **Command execution** - Type and execute any system command
- **Real-time output** display with color-coded text
- **Clear terminal** button to reset output
- **Cross-platform** - Works on Windows, macOS, and Linux

### ▶️ File Execution
- **Run file button** - Execute files directly:
  - Rust files (`.rs`) - Compile and run with `rustc`
  - Python files (`.py`) - Execute with `python`
  - JavaScript files (`.js`) - Run with `node`
- **Build button** - Build Rust projects with `cargo build --release`
- **Live output** in terminal with error reporting

### 🎨 User Interface
- **Professional dark theme** (Dracula-inspired)
- **Color-coded UI elements**:
  - Cyan (`#8BE9FD`) for accents
  - Green (`#50FA7B`) for success/positive actions
  - Pink/Red (`#FF79C6`) for errors/danger
  - Gray (`#626EA0`) for muted/secondary text
- **Responsive layout** with resizable panels
- **Smooth scrolling** for code and terminal
- **Emoji icons** for visual appeal and quick recognition

### 📋 File Type Support
Recognized file types with custom icons:
- 🦀 Rust (`.rs`)
- 🐍 Python (`.py`)
- 📜 JavaScript/TypeScript (`.js`, `.jsx`, `.ts`, `.tsx`)
- 🌐 HTML (`.html`)
- 🎨 CSS/SCSS (`.css`, `.scss`)
- 📦 JSON (`.json`)
- 📝 Markdown (`.md`)
- 📄 Text (`.txt`)
- ⚙️ Config files (`.toml`, `.yaml`, `.yml`)
- 🔒 Lock files (`.lock`)

---

## 🚀 Getting Started

### Prerequisites
- **Rust 1.70+** installed
- **Cargo** package manager

### Installation

1. **Clone or download** this repository:
```bash
cd code_editor
```

2. **Build the project**:
```bash
cargo build --release
```

3. **Run the application**:
```bash
cargo run --release
```

Or directly execute the binary:
```bash
./target/release/code_editor
```

---

## 📖 Usage Guide

### Opening Files and Folders

1. **Open a Folder**:
   - Click `📁 File` → `📂 Open Folder`
   - Select a directory to browse
   - All files and subdirectories will appear in the left sidebar

2. **Open a File**:
   - Click `📁 File` → `📄 Open File`
   - Or click any file in the file tree to open it
   - Files open as new tabs at the top

3. **Navigate Files**:
   - Use the **breadcrumb navigation** at the top to jump between folders
   - Click folder names in the breadcrumb to go back

### Editing Code

1. **Type Code**:
   - Click in the editor area and start typing
   - Line numbers appear automatically on the left
   - Code is displayed in monospaced font for clarity

2. **Auto-Completion**:
   - Type `(` and it automatically adds `)`
   - Type `[` and it automatically adds `]`
   - Type `{` and it automatically adds `}`
   - Type `"` and it automatically adds `"`
   - Type `'` and it automatically adds `'`

3. **Save Files**:
   - Click `📁 File` → `💾 Save` to save changes
   - Or use keyboard shortcut (if configured)
   - Success message appears in terminal

4. **Close Tabs**:
   - Click the **✕** button on any tab to close it
   - Or use the **Close** button in tabs area

### Running Code

1. **Run a File**:
   - Click **▶️ Run** button at the top
   - Supported formats: `.rs`, `.py`, `.js`
   - Output appears in the terminal below

2. **Build Rust Projects**:
   - Click **🔨 Build** button to build with `cargo build --release`
   - Build output displays in terminal

### Using the Terminal

1. **Open Terminal**:
   - Terminal opens automatically at the bottom
   - Or toggle with `👁️ View` → `🖥️ Show Terminal`

2. **Start PowerShell or CMD**:
   - Click **⚡ PowerShell** to launch PowerShell
   - Click **⚡ CMD** to launch Windows Command Prompt

3. **Execute Commands**:
   - Type command in the input field at the bottom
   - Click **Execute ▶** to run the command
   - Output displays above in the terminal area

4. **Clear Terminal**:
   - Click **🗑️ Clear** to clear all terminal output

### Example Commands

```bash
# List files
dir                    # Windows
ls                     # Unix/Mac

# Show a message
echo Hello, World!

# Run Python script
python script.py

# Execute Node.js
node app.js

# Check Rust version
rustc --version

# Build Rust project
cargo build

# Navigate directories
cd path/to/folder
```

---

## 🎹 Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Open Folder | - |
| Open File | - |
| Save File | - |
| Exit Application | Click ❌ Exit |
| Toggle Sidebar | 👁️ View → 📁 Hide/Show Sidebar |
| Toggle Terminal | 👁️ View → 🖥️ Hide/Show Terminal |
| Execute Command | Click Execute ▶ |
| Clear Terminal | Click 🗑️ Clear |

---

## 📁 Project Structure

```
code_editor/
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # Application state
│   ├── ui.rs             # UI rendering
│   ├── editor.rs         # Code editor logic
│   ├── file_tree.rs      # File explorer
│   ├── file_ops.rs       # File operations
│   └── terminal.rs       # Terminal integration
├── Cargo.toml            # Dependencies
└── README.md             # This file
```

---

## 🛠️ Dependencies

- **eframe** (0.22) - GUI framework
- **egui** (0.22) - Immediate mode GUI library
- **tokio** (1.0) - Async runtime
- **rfd** (0.10) - File/folder dialogs
- **syntect** (5.0) - Syntax highlighting (future use)
- **walkdir** (2) - Directory traversal
- **winapi** (0.3) - Windows API bindings

---

## 🎨 Color Scheme (Dracula)

| Element | Color | Hex |
|---------|-------|-----|
| Background | Dark Gray | `#282A36` |
| Sidebar | Darker Gray | `#1C1E2B` |
| Accent | Cyan | `#8BE9FD` |
| Success | Green | `#50FA7B` |
| Error | Pink | `#FF79C6` |
| Muted | Gray | `#626EA0` |
| Text | Light | `#F8F8F2` |

---

## 🔄 Built-in Commands

### File Commands
- Open Folder
- Open File
- Save File
- Exit Application

### View Commands
- Toggle Sidebar
- Toggle Terminal

### Terminal Commands
- Open PowerShell
- Open CMD
- Execute Command
- Clear Terminal

### Editor Commands
- Run File (Rust, Python, Node.js)
- Build Project (Cargo)

---

## 📝 Tips & Tricks

1. **Quick Navigation**: Use breadcrumbs to jump between folders without scrolling
2. **Multiple Files**: Open multiple files using tabs for easy switching
3. **Terminal History**: Terminal output persists - scroll up to see previous commands
4. **File Icons**: Different icons for different file types help quick identification
5. **Auto-Save**: Save frequently to avoid data loss
6. **Clear Terminal**: Clear terminal output when it gets cluttered for better readability

---

## 🐛 Troubleshooting

### Terminal Not Opening
- Make sure PowerShell or CMD is installed
- Check Windows PATH includes the terminal executable
- Try clicking the buttons again

### Files Not Showing
- Make sure you opened a folder, not just individual files
- Refresh the folder view by closing and reopening the folder

### Commands Not Executing
- Check command syntax
- Ensure Python, Node.js, or Rust is installed and in PATH
- Check terminal output for error messages

### Application Crashes
- Update Rust: `rustc --version` and `rustup update`
- Rebuild: `cargo clean && cargo build --release`
- Check console output for error details

---

## 🚀 Performance

- **Lightweight**: Uses immediate-mode GUI for efficient rendering
- **Multi-threaded**: Terminal commands run on separate threads
- **Responsive**: UI remains responsive during file operations
- **Memory Efficient**: Optimized for modern systems

---

## 📜 License

This project is open-source and available for personal and educational use.

---

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

---

## 📚 Future Enhancements

- [ ] Full syntax highlighting with color-coded keywords
- [ ] Code snippets and autocomplete
- [ ] Search and replace functionality
- [ ] Debugger integration
- [ ] Git integration
- [ ] Theme customization
- [ ] Plugin system
- [ ] Settings/preferences panel
- [ ] Code formatter support
- [ ] Language server protocol (LSP) support

---

## 🎯 Version

**Version**: 0.2.0
**Last Updated**: November 2025

---

## 👨‍💻 Author

Built with ❤️ using Rust and egui

---

## 💬 Support

For issues, questions, or suggestions, feel free to create an issue or contact the development team.

---

**Happy Coding! 🚀**
