<p align="center">
  <h1 align="center">urll</h1>
  <p align="center">
    <a href="https://github.com/dhth/urll/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/urll/main.yml?style=flat-square"></a>
  </p>
</p>

`urll` lets you browse URLs on a webpage in a recursive manner.

![tui](https://github.com/user-attachments/assets/b1939f41-e3b8-4e8a-8e28-a27a9fe39133)

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/urll.git
```

⚡️ Usage
---

```text
Usage: urll [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -t, --tui
  -h, --help  Print help
```

📟 TUI
---

![urll](https://github.com/user-attachments/assets/6670724f-93e2-4750-b99b-9f74e5aeb600)

`urll` lets you browse results in a TUI. You can interactively view results for
a URL returned as a result of a previous URL, creating a history of viewed
results in the process.

### TUI Keymaps

| Keymap        | Action                                    |
|---------------|-------------------------------------------|
| `j` / `Down`  | go down                                   |
| `k` / `Up`    | go up                                     |
| `Enter`       | show results for URL under cursor         |
| `g`           | go to the top                             |
| `G`           | go to the end                             |
| `<backspace>` | go back in navigation history             |
| `y`           | yank URL under cursor to system clipboard |
| `Y`           | yank all URLs to system clipboard         |
| `o`           | open URL under cursor in browser          |
| `?`           | show/hide help view                       |
| `Esc` / `q`   | go back/quit                              |
| `<ctrl+c>`    | quit immediately                          |
