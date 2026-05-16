<h1 align="center">
 kadepayd
</h1>
<p align="center">
<a href="https://github.com/KadePayments/kadepayd/actions/workflows/build.yml">
<img src="https://github.com/KadePayments/kadepayd/actions/workflows/build.yml/badge.svg" alt="CI status">
</a>
</p>

## Overview
**kadepayd** is a daemon server to power **KadePay**, a self-hosted payment processor for accepting off-chain Bitcoin 
payments powered by Arkade.

⚠ **Note:** This project is under heavy development, do not use for production.

---

## Architecture

| Modules | Description       |
|---------|-------------------|
| src     | Main  source code |
| tests   | Tests code        |
| protos  | Protocol buffers  |
| scripts | Utility scripts   |

<p align="center">
<img src="https://github.com/KadePayments/KadePay/blob/main/assets/architecture.png" alt="architecture">
</p>

---

## Getting Started

### Prerequisites
- **Rust 1.9+**

### Installation
Clone the repository:
```bash
git clone https://github.com/KadePayments/kadepayd.git
cd kadepayd
```

Build the project:
```bash
cargo build
```

Run the daemon:
```bash
cargo run --package kadepayd --bin kadepayd
```
---

## Contributing
Contributions are welcome!
- Fork the repo
- Create a feature branch
- Submit a pull request

Please note that this project is **experimental**, so expect frequent changes.

### Development Environment

#### Setup Pre-commit Hook
```shell
cp scripts/pre-commit .git/hooks/
```

---

## License
This project is licensed under the **MIT License**. See the [License](./LICENSE) file for details.

---

## Disclaimer
This project is **experimental** and should **not** be used in production environments. Use at your own risk.

---