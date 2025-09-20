# PostgreSQL CRUD Application in Rust

A CLI-based CRUD (Create, Read, Update, Delete) application built with Rust and PostgreSQL using SQLx for database operations. Features automatic database creation and migration system.

## Features

- **Complete CRUD Operations**: Create, read, update, and delete user records
- **Auto-Migration**: Automatically creates database and tables on first run
- **Sample Data**: Pre-loads sample user data for immediate testing
- **Interactive CLI**: User-friendly command-line interface
- **Type-Safe Database Operations**: Uses SQLx for compile-time verified queries
- **Decimal Precision**: Accurate height measurements using rust_decimal

## Prerequisites

- **Rust** (latest stable version)
- **PostgreSQL** server installed and running
- **Git** for cloning the repository

## Quick Start

### 1. Clone Repository
```bash
git clone <your-repository-url>
cd db_crud_pg
```

### 2. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your PostgreSQL credentials
nano .env  # or vim .env
```

### 3. Configure Database Connection
Update `.env` file with your PostgreSQL settings:
```env
DATABASE_URL="postgresql://username:password@localhost:5432/crud_db"
```

Example for default setup:
```env
DATABASE_URL="postgresql://admin:admin123@localhost:5432/crud_db"
```

### 4. Run Application
```bash
# Build and run (auto-migration will handle database setup)
cargo run --release --bin pg_cli
```

The application will automatically:
- Create the database if it doesn't exist
- Run migrations to create the `users` table
- Insert sample data for immediate testing
- Launch the interactive CLI

## Database Schema

The application creates a `users` table with the following structure:

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | SERIAL | PRIMARY KEY | Auto-incrementing user ID |
| name | VARCHAR(100) | NOT NULL | User's full name |
| age | INTEGER | NOT NULL | User's age in years |
| height | NUMERIC(5,2) | NOT NULL | User's height in cm (e.g., 175.50) |

## CLI Menu Options

```
Choose CRUD mode operations:
1. Insert user (name, age, height)    - Add new user
2. Select user by id (id)             - Find user by ID
3. Update user age (id)               - Modify user's age
4. Delete user (id)                   - Remove user
5. Get all users data                 - Display all users
6. Exit                               - Quit application
```

## Sample Data

The application comes pre-loaded with 12 sample users:
- Faishal (24, 161.00cm)
- Fahmi (16, 169.00cm)
- Farham (23, 165.00cm)
- And 9 more...

## Manual Database Setup (Optional)

If you prefer manual database setup or face issues with auto-migration:

### PostgreSQL Setup
```bash
# Start PostgreSQL server
pg_ctl -D pg-data start

# Create database manually
createdb -U your_username crud_db

# Connect to database
psql -U your_username -d crud_db
```

### Manual Migration
```bash
# Run migrations manually using SQLx CLI
cargo install sqlx-cli
sqlx migrate run --database-url "your_database_url"
```

## Project Structure

```
db_crud_pg/
├── migrations/
│   └── 001_create_users_table.sql    # Database migration with sample data
├── src/
│   ├── main.rs                       # Application entry point with auto-migration
│   ├── lib.rs                        # Library exports and module declarations
│   ├── db.rs                         # Database models and connection setup
│   ├── models.rs                     # User model with CRUD implementations
│   └── cli.rs                        # Interactive CLI interface
├── .env                              # Environment variables (not in git)
├── .env.example                      # Environment template
├── .gitignore                        # Git ignore rules
├── Cargo.toml                        # Rust dependencies and build config
└── README.md                         # This documentation
```

## Development Commands

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running
```bash
# Debug run
cargo run --bin pg_cli

# Release run (faster)
cargo run --release --bin pg_cli
```

### Database Management
```bash
# Create new migration
sqlx migrate add create_new_table

# Run all pending migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

### Cleaning
```bash
# Clean build artifacts (saves ~520MB)
cargo clean
```

## Dependencies

- **tokio** `1.x` - Async runtime for handling database operations
- **sqlx** `0.7` - Type-safe SQL toolkit with PostgreSQL support and migrations
- **dotenv** `0.15` - Environment variable loading from `.env` files
- **rust_decimal** `1.0` - Precise decimal arithmetic for height measurements
- **chrono** `0.4` - Date and time handling (for future features)

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://admin:password@localhost:5432/crud_db` |

## Error Handling

The application handles common errors gracefully:
- **Database connection failures**: Clear error messages with setup hints
- **Invalid input**: User-friendly validation messages
- **Missing records**: Informative "not found" responses
- **Type conversion errors**: Helpful format guidance

## Testing

### Test Auto-Migration
To verify the auto-migration feature works correctly:

```bash
# 1. Stop PostgreSQL
pg_ctl -D pg-data stop

# 2. Backup existing database
mv pg-data pg-data-backup

# 3. Initialize fresh database
initdb -D pg-data -U admin

# 4. Start PostgreSQL
pg_ctl -D pg-data start

# 5. Run application (should auto-create everything)
cargo run --release --bin pg_cli

# 6. Restore original database
pg_ctl -D pg-data stop
rm -rf pg-data
mv pg-data-backup pg-data
pg_ctl -D pg-data start
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test thoroughly
4. Commit changes: `git commit -m "Add feature description"`
5. Push to branch: `git push origin feature-name`
6. Submit a Pull Request

## Troubleshooting

### Common Issues

**Database Connection Refused**
```bash
# Check if PostgreSQL is running
pg_ctl -D pg-data status

# Start PostgreSQL if not running
pg_ctl -D pg-data start
```

**Migration Fails**
```bash
# Check database exists and is accessible
psql -U username -d crud_db -c "SELECT version();"

# Manually run migrations
sqlx migrate run --database-url "your_database_url"
```

**Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

## License

This project is open source and available under the [MIT License](LICENSE).

## Author

Built as a learning project to demonstrate Rust database operations with PostgreSQL, featuring automatic database setup and migrations for easy deployment.
