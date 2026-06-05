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
- **Rust 1.85+**
- **PostgreSQL 18.2+**

### Installation

#### Clone the repository:
```bash
git clone https://github.com/KadePayments/kadepayd.git
cd kadepayd
```

#### Deployment Environment Setup:
There two ways to set up the deployment environment 

1. Environment variables (Recommended for production)

    | Variable                | Description                                                                                                                   |
    |-------------------------|-------------------------------------------------------------------------------------------------------------------------------|
    | `KADEPAY_HOST`          | The IP address of the host machine, set automatically if not defined; use this to explicitly set the host IP                  |
    | `KADEPAY_INVOICES_PORT` | The port exposing KadePay invoices service                                                                                    |
    | `KADEPAY_DB_HOST`       | The IP address of the host of the PostgreSQL database to use for data storage; use this as an alternative to `KADEPAY_DB_URL` |
    | `KADEPAY_DB_URL`        | The URL for the PostgreSQL database to use for data storage; use this as an alternative to `KADEPAY_DB_HOST`                  |
    | `KADEPAY_DB_USER`       | The username for the PostgreSQL database                                                                                      |
    | `KADEPAY_DB_NAME`       | The PostgreSQL database name                                                                                                  |
    | `KADEPAY_DB_PASSWORD`   | The PostgreSQL database password                                                                                              |

2. Create a `.secrets` file (For testing and development)

    **Syntax**
    ```dotenv
    KADEPAY_DB_HOST=localhost
    KADEPAY_DB_USER=postgres
    KADEPAY_DB_PASSWORD=kadepay_db_password
    KADEPAY_DB_NAME=postgres
    KADEPAY_INVOICES_PORT=50051    ```
#### Build the project:
```bash
cargo build
```

#### Run the daemon:
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