<div align="center">
<h1>🧹 Clean My Keys</h1>
</div>

A sleek, minimalist utility, built entirely in **Rust** using the **Iced** GUI framework. It allows you to temporarily disable and lock your keyboard with a single click so you can safely wipe it down without triggering unwanted commands or chaotic inputs.

## ✨ Features

- **One-Click Lock:** Instantly intercept and safely block all keyboard inputs.
- **Device Selection:** Choose exactly which keyboard to disable using Linux `evdev` subsystem integration.
- **Modern UI:** A clean, native-looking interface design
- **Dynamic Themes:** Toggle between various built-in themes (Dark, Dracula, Tokyo Night, Gruvbox, etc.) on the fly.
- **Lightweight & Safe:** Blazing fast performance with zero electron bloat, thanks to Rust.

## 🚀 How It Works (The Tech Stack)

Under the hood, KeyClean leverages the Linux kernel's **`evdev`** subsystem. When you press "Start", the app triggers an exclusive `grab()` on the selected device file (found under `/dev/input/event*`). This tells the kernel to route all keystrokes exclusively to KeyClean (where they are swallowed safely) until you hit stop. Your mouse remains entirely free (unless you've selected your mouse from the dropdown menu - yes, you can do that!) so you can interact with the UI at any time.

## 📦 Installation & Prerequisites

The app will try to elevate itself with `pkexec` on start if it does not have access to `/dev/input/`. If your system blocks that, you can still grant access manually via the `input` group.

### Option 1: Build from source

Then clone and build the project:
```bash
git clone https://github.com/<your-user>/clean-my-keys.git
cd clean-my-keys
cargo build --release
```

Run the binary from `target/release/clean-my-keys`.

### Option 2: Install from AUR

https://aur.archlinux.org/packages/clean-my-keys

If you prefer an AUR package, install it with your AUR helper once a package is available:
```bash
yay -S clean-my-keys
```

If you do not use an AUR helper, you can also build the package manually with `makepkg -si` from the AUR PKGBUILD directory.
