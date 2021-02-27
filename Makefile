include: .env
export

watch:
	RUST_LOG=debug cargo watch -w src -x "lrun"
seed:
	sqlite3 -csv seed_data/users.csv ".import $$DATABASE_URL users"
