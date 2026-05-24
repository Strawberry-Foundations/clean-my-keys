<div align="center">
<h1>🧹 Clean My Keys</h1>
</div>

A sleek, minimalist utility, built entirely in **Rust** using the **Iced** GUI framework. It allows you to temporarily disable and lock your keyboard with a single click so you can safely wipe it down without triggering unwanted commands or chaotic inputs.

<p align="center">
  </p>

## ✨ Features

- **One-Click Lock:** Instantly intercept and safely block all keyboard inputs.
- **Device Selection:** Choose exactly which keyboard to disable using Linux `evdev` subsystem integration.
- **Modern UI:** A clean, native-looking interface design
- **Dynamic Themes:** Toggle between various built-in themes (Dark, Dracula, Tokyo Night, Gruvbox, etc.) on the fly.
- **Lightweight & Safe:** Blazing fast performance with zero electron bloat, thanks to Rust.

## 🚀 How It Works (The Tech Stack)

Under the hood, KeyClean leverages the Linux kernel's **`evdev`** subsystem. When you press "Start", the app triggers an exclusive `grab()` on the selected device file (found under `/dev/input/event*`). This tells the kernel to route all keystrokes exclusively to KeyClean (where they are swallowed safely) until you hit stop. Your mouse remains entirely free (unless you've selected your mouse from the dropdown menu - yes, you can do that!) so you can interact with the UI at any time.

## 📦 Installation & Prerequisites

Since Linux locks down raw device inputs for security purposes to prevent keyloggers, you need appropriate permissions to interact with `/dev/input/`.

### 1. Add your user to the `input` group:
To run the app without typing `sudo` every time, add your user to the input group and **log out and back in** (or restart):
```bash
sudo usermod -aG input $USER
```