# sqlx-sandbox

Sandbox using [launchbadge/sqlx](https://github.com/launchbadge/sqlx)

## Development

```bash
# setup DB server (currently MariaDB)
docker compose up -d
# configure environment variable
export DATABASE_URL="mysql://root:root@localhost:3306/db"
# run codes
cargo run
# or tests
cargo test
```
