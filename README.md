<div align="center">
  <img src="apps/launcher-rust/app-icon.png" alt="Zapuskalka Logo" width="128" height="128">
  
  # Zapuskalka
  
  **Open-source launcher and platform for distributing indie games and applications**
  
  [![Latest Release](https://img.shields.io/github/v/release/rodd-oss/zapuskalka?label=Latest%20Release&style=for-the-badge)](https://github.com/rodd-oss/zapuskalka/releases/latest)
</div>

## Передача прав / Rights transfer

Проект Zapuskalka продолжается как закрытое программное обеспечение. Авторов, чей код есть в этом архиве, просят подписать [Соглашение об отчуждении исключительного права](legal/RIGHTS-TRANSFER.md), добавив файл в [`legal/signatures/`](legal/signatures/README.md) со своей учётной записи GitHub. Лицензия самого архива (MPL 2.0) не меняется.

Zapuskalka continues as closed-source software. Contributors whose code is in this archive are asked to sign the [Copyright Assignment Agreement](legal/RIGHTS-TRANSFER.md) by adding a file under [`legal/signatures/`](legal/signatures/README.md) from their own GitHub account. The archive's own licence (MPL 2.0) does not change.

## About

Zapuskalka is a modern open-source launcher designed for indie game players and developers. The project combines the ease of use of Steam with the open distribution model of itch.io, providing a free and fair platform for indie content distribution.

### Key Features

- 🎮 **Game Library** — manage all your indie games in one place
- 🛒 **Game Store** — open marketplace with search and filtering
- 📦 **Installation & Updates** — automatic game installation and updates
- 🔐 **Secure Authentication** — sign in via OAuth (GitHub/Twitch) or email
- 🎨 **Modern Interface** — intuitive UI built with Vue 3
- 🚀 **Cross-platform** — support for Windows, macOS, and Linux

## Download

### Latest Release

Download the latest version for your platform from the [releases page](https://github.com/rodd-oss/zapuskalka/releases/latest):

- **Windows** — Look for `.exe` (installer) or `.msi` files (recommended)
- **macOS** — Look for `.dmg` files (for Apple Silicon or Intel)
- **Linux** — Look for `.AppImage` or `.deb` files

📦 [All releases and versions](https://github.com/rodd-oss/zapuskalka/releases)

### macOS Installation

After downloading and installing the application:

1. Drag Zapuskalka to the Applications folder
2. Open Terminal
3. Run the following command to remove the quarantine attribute:

```bash
sudo xattr -r -c /Applications/Zapuskalka.app
```

The application is now ready to launch!

---

## Development Guide

### Requirements

Before starting development, make sure you have installed:

- **Node.js** 20.19+ or 22.12+ — [Download](https://nodejs.org/en/download)
- **Go** 1.25.1+ — [Installation guide](https://go.dev/doc/install)
- **Rust** — [Installation guide](https://www.rust-lang.org/tools/install)
- **Bun** — [Installation guide](https://bun.com/docs/installation)
- **Taskfile** — [Installation guide](https://taskfile.dev/docs/installation#get-the-binary)

### Install Dependencies

Run in the project root:

```bash
bun install
```

### Create Backend Admin Account

1. Start the backend:

```bash
task dev-backend
```

2. Open the URL from the terminal and create a local admin account
3. Navigate to the users table, enable and create a user account
4. Stop the backend service

### Run Development Mode

To run the entire project in development mode:

```bash
task dev
```

---

## Useful Links

### Documentation
- 📖 [Product Documentation](docs/PRODUCT.md) — Product requirements and feature specifications
- 🏗️ [Project Architecture](docs/ARCHITECTURE.md) — Technical architecture and system design

### Community & Support
- 🐛 [Report a Bug](https://github.com/rodd-oss/zapuskalka/issues/new?template=bug_report.yml) — Found a bug? Let us know!
- 💡 [Request a Feature](https://github.com/rodd-oss/zapuskalka/issues/new) — Have an idea? Share it with us!
- 🔄 [Changelog](https://github.com/rodd-oss/zapuskalka/releases) — See what's new in each release

### Contributing
- 🤝 [Contributing Guide](https://github.com/rodd-oss/zapuskalka/blob/main/CONTRIBUTING.md) — Learn how to contribute (in development)
- 📝 [Code of Conduct](https://github.com/rodd-oss/zapuskalka/blob/main/CODE_OF_CONDUCT.md) — Community guidelines (in development)
