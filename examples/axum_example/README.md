![screenshot](Screenshot.png)

![screenshot](https://www.sea-ql.org/sea-orm-pro/assets/images/getting-started-axum-04-raw-tables-posts-b58b15f52a755c37c00f12ad6d8efeb5.png)

![screenshot](<Screenshot GraphQL.png>)

# Axum with SeaORM example app

1. Modify the `DATABASE_URL` var in `.env` to point to your chosen database

    Turn on the appropriate database feature for your chosen db in `service/Cargo.toml` (the `"sqlx-postgres",` line)

1. Download admin frontend with `sh ../../build_tools/download_frontend.sh`

1. Execute `cargo run` to start the server

1. Visit in browser:

    + [Frontend](http://localhost:8000)
    + [Admin](http://localhost:8000/admin)
    + [GraphQL playground](http://localhost:8000/api/graphql)

1. (optional) Run tests on the service logic crate:

```bash
cd service
cargo test
```
