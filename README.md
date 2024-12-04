# bville-recycle ♻️
[![Built with Rust](static/built_with_rust.svg)](https://www.rust-lang.org)
[![unsafe forbidden](static/unsafe_%20forbidden.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT license](static/license_%20MIT.svg)](/LICENSE)

A Rust-based web app to show up-to-date information on publicly-available recycling programs within the Bartlesville, OK city limits.

**Project Purpose:** Promote environmental sustainability by making recycling more accessible and user-friendly for residents of Bartlesville, OK through the use of information aggregation on local recycling programs. 

A project of [Phishing Spider LLC](https://github.com/PhishingSpider).

## Technology Stack
- **Programming Language:** Rust
- **Database:** MariaDB 
- **Frontend Framework:** TBD
- **Maps:** OpenStreetMap (with attribution and compliance)

## Project Roadmap
1. [x] Compose [REQUIREMENTS.md](./REQUIREMENTS.md)
2. [ ] Develop platform
3. [ ] Gather data for MVP (identify key recycling sites and related information)
4. [ ] Test, optimize, and audit the platform
5. [ ] Deploy platform
    1. [ ] Ensure all administrative credentials get changed away from defaults. 
6. [ ] Spread awareness
7. [ ] Update and maintain the platform as needed

## Setup

Follow these steps to get started with `bville-recycle`:

---

### Prerequisites

Before you begin, ensure the following:
1. **Rust Installed:**
   - Install Rust using [rustup](https://rustup.rs/):
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - Verify installation:
     ```bash
     rustc --version
     cargo --version
     ```

2. **MariaDB Installed:**
   - Follow the [MariaDB installation guide](https://mariadb.org/download/), or use your system’s package manager.
     Example for Ubuntu:
     ```bash
     sudo apt update
     sudo apt install mariadb-server
     ```

3. **Git Installed:**
   - Ensure Git is installed. Example for Ubuntu:
     ```bash
     sudo apt install git
     ```
   - Check installation:
     ```bash
     git --version
     ```

---

### Step 1: Clone the Repository
Open a terminal and run:
```bash
git clone https://github.com/PhishingSpider/bville-recycle.git
cd bville-recycle
```

### Step 2: Configure Your Environment
Create a `.env` file in the project root with your MariaDB credentials. Use non-default credentials for better security:
```
DATABASE_URL=mysql://your_username:your_password@localhost/bville_recycle
```

### Step 3: Set Up the Database

Open the MariaDB command-line client and run the following commands:

```sql
CREATE DATABASE bville_recycle;
CREATE USER 'your_username'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON bville_recycle.* TO 'your_username'@'localhost';
FLUSH PRIVILEGES;
```

If you're unsure, refer to the `Set up MariaDB` section of our [GitHub Actions workflow for Rust](./.github/workflows/rust.yml) for a practical example.

### Step 4: Build and Run the Application

Build the project:

```bash
cargo build
```

Run the application: 

```bash
cargo run
```

The application will start, and you can access it locally at http://localhost:8000. (TLS pending)

### Step 5: Run Tests (Optional)

To ensure everything is set up correctly:
```bash
cargo test
```

You're all set! If you encounter any issues, check our [CONTRIBUTING.md](./CONTRIBUTING.md) for support or open an issue.

## Contributing
Contributions are welcome! Please review our [CONTRIBUTING.md](./CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) files for guidelines.

## License
This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
