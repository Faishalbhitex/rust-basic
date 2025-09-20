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
git clone https://github.com/Faishalbhitex/rust-basic.git
cd rust-basic/db_crud_pg
```

### 2. PostgreSQL Setup (First Time Only)

If you don't have PostgreSQL set up yet:

```bash
# Create PostgreSQL data directory
mkdir -p $HOME/pg-data

# Initialize PostgreSQL database cluster
initdb -D $HOME/pg-data -U admin

# Start PostgreSQL server
pg_ctl -D $HOME/pg-data start

# The server will create logs in your directory
```

### 3. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env with your PostgreSQL credentials
nano .env  # or vim .env
```

### 4. Configure Database Connection
Update `.env` file with your PostgreSQL settings:
```env
DATABASE_URL="postgresql://admin:your_password@localhost:5432/crud_db"
```

Example configuration:
```env
DATABASE_URL="postgresql://admin:admin123@localhost:5432/crud_db"
```

### 5. Run Application
```bash
# Build and run (auto-migration will handle database setup)
cargo run --release --bin pg_cli
```

The application will automatically:
- Create the database `crud_db` if it doesn't exist
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
- Nasrul (23, 170.00cm)
- Andi (27, 178.00cm)
- Ahmad Saputra (27, 170.00cm)
- Andrean (27, 182.00cm)
- Fadhil (24, 161.00cm)
- Jamal (30, 160.00cm)
- Taufik (24, 165.00cm)
- Ilham (25, 160.00cm)
- Firman (24, 165.00cm)

## Manual Database Setup (Alternative)

If you prefer manual database setup or face issues with auto-migration:

### PostgreSQL Management
```bash
# Check PostgreSQL status
pg_ctl -D $HOME/pg-data status

# Start PostgreSQL server
pg_ctl -D $HOME/pg-data start

# Stop PostgreSQL server
pg_ctl -D $HOME/pg-data stop

# Connect to database
psql -U admin -d crud_db -h localhost
```

### Manual Migration
```bash
# Install SQLx CLI (if not already installed)
cargo install sqlx-cli

# Run migrations manually
sqlx migrate run --database-url "postgresql://admin:password@localhost:5432/crud_db"
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

## Testing Auto-Migration

To verify the auto-migration feature works correctly:

```bash
# 1. Stop PostgreSQL
pg_ctl -D $HOME/pg-data stop

# 2. Backup existing database
mv $HOME/pg-data $HOME/pg-data-backup

# 3. Initialize fresh database
initdb -D $HOME/pg-data -U admin

# 4. Start PostgreSQL
pg_ctl -D $HOME/pg-data start

# 5. Run application (should auto-create everything)
cargo run --release --bin pg_cli

# 6. Restore original database (optional)
pg_ctl -D $HOME/pg-data stop
rm -rf $HOME/pg-data
mv $HOME/pg-data-backup $HOME/pg-data
pg_ctl -D $HOME/pg-data start
```

## Troubleshooting

### Common Issues

**Database Connection Refused**
```bash
# Check if PostgreSQL is running
pg_ctl -D $HOME/pg-data status

# Start PostgreSQL if not running
pg_ctl -D $HOME/pg-data start
```

**PostgreSQL Not Initialized**
```bash
# Initialize PostgreSQL data directory
initdb -D $HOME/pg-data -U admin
pg_ctl -D $HOME/pg-data start
```

**Migration Fails**
```bash
# Check database exists and is accessible
psql -U admin -d crud_db -c "SELECT version();"

# Manually run migrations
sqlx migrate run --database-url "postgresql://admin:password@localhost:5432/crud_db"
```

**Build Errors**
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

**Permission Issues**
- Ensure the user specified in DATABASE_URL has appropriate permissions
- Check that PostgreSQL is configured to accept local connections

## About This Project

This project is part of a monorepo collection of Rust learning projects. It demonstrates:
- Database operations with SQLx
- Async programming in Rust
- CLI application development
- Database migrations and auto-setup
- Error handling and user input validation

## Repository Context

This project is located within the [rust-basic](https://github.com/Faishalbhitex/rust-basic) repository, which contains various Rust learning projects and examples.

## License

This project is open source and available under the [MIT License](LICENSE).

## Author

Built as a learning project to demonstrate Rust database operations with PostgreSQL, featuring automatic database setup and migrations for easy deployment.
