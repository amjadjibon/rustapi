migrate:
	cargo sqlx migrate run --database=${DATABASE_URL}
