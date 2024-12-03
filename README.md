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
After cloning the repo. Set up the MariaDB database. Ensure you have a user set up with non-default credentials (did I mention non-default?) which match those listed in [Rocket.toml](./Rocket.toml) and and [src/lib.rs `async fn`](./src/lib.rs). 


```sql
CREATE USER 'username'@'localhost' IDENTIFIED BY 'password';
CREATE DATABASE bville_recycle;
GRANT ALL PRIVILEGES ON bville_recycle.* TO 'username'@'localhost';
FLUSH PRIVILEGES;
```


## Contributing
Contributions are welcome! Please review our [CONTRIBUTING.md](./CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md) files for guidelines.

## License
This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
