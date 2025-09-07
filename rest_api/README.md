# Todo REST API with Rust & Axum

A simple yet efficient REST API built with Rust using the Axum web framework. This project demonstrates modern Rust web development practices including async programming, JSON serialization, and file-based persistence.

## 🚀 Features

- **Full CRUD Operations**: Create, Read, Update, and Delete todos
- **JSON Persistence**: Data automatically saved to/loaded from `todos.json`
- **Async/Await**: Built with Tokio for high-performance async operations
- **Type Safety**: Leverages Rust's type system for robust API contracts
- **Error Handling**: Proper HTTP status codes and error responses

## 🛠️ Technology Stack

- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Axum](https://github.com/tokio-rs/axum)** - Modern web framework for Rust
- **[Tokio](https://tokio.rs/)** - Asynchronous runtime
- **[Serde](https://serde.rs/)** - Serialization/deserialization framework

## 📋 API Endpoints

| Method | Endpoint     | Description           | Request Body                    |
|--------|-------------|-----------------------|--------------------------------|
| GET    | `/todos`     | Get all todos         | -                              |
| POST   | `/todos`     | Create a new todo     | `{"title": "string"}`          |
| GET    | `/todos/:id` | Get specific todo     | -                              |
| PUT    | `/todos/:id` | Update existing todo  | `{"title"?: "string", "completed"?: boolean}` |
| DELETE | `/todos/:id` | Delete todo           | -                              |

## 🏗️ Project Structure

```
src/
├── main.rs          # Server setup and routing
├── handlers.rs      # HTTP request handlers
├── models.rs        # Data structures and DTOs
└── storage.rs       # File I/O operations and data management
```

## 🚦 Getting Started

### Prerequisites

- Rust 1.75+ installed ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation & Setup

1. **Clone the repository**
   ```bash
   git clone <your-repo-url>
   cd rest_api
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run the server**
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:3000`

### Testing the API

#### Manual Testing with curl

```bash
# Get all todos
curl -X GET http://127.0.0.1:3000/todos

# Create a new todo
curl -X POST http://127.0.0.1:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust"}'

# Get specific todo
curl -X GET http://127.0.0.1:3000/todos/1

# Update todo
curl -X PUT http://127.0.0.1:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# Delete todo
curl -X DELETE http://127.0.0.1:3000/todos/1
```

#### Automated Testing

Run the comprehensive test script:

```bash
chmod +x scripts/test_api.sh
./scripts/test_api.sh
```

## 📊 Data Model

### Todo
```rust
{
  "id": u32,           // Auto-generated unique identifier
  "title": String,     // Todo description
  "completed": bool    // Completion status
}
```

### Create Todo Request
```rust
{
  "title": String      // Required: Todo description
}
```

### Update Todo Request
```rust
{
  "title"?: String,    // Optional: New todo description
  "completed"?: bool   // Optional: New completion status
}
```

## 🎯 Learning Objectives

This project covers essential Rust web development concepts:

- **Ownership & Borrowing**: Understanding Rust's memory management
- **Async Programming**: Using `async/await` with Tokio
- **Error Handling**: Rust's `Result<T, E>` pattern
- **Serialization**: JSON handling with Serde
- **Web Frameworks**: Building APIs with Axum
- **State Management**: Shared state with `Arc<Mutex<T>>`
- **File I/O**: Persistent data storage

## 🔧 Key Implementation Details

### Shared State Management
Uses `Arc<Mutex<Storage>>` for thread-safe shared state across async handlers.

### Persistence Strategy
- Data loaded from `todos.json` on startup
- Auto-save after each modification
- Graceful handling of missing/corrupted data files

### Error Handling
- Proper HTTP status codes (404, 201, 204)
- Rust's `Result` type for error propagation
- JSON serialization error handling

## 🚀 Future Enhancements

- [ ] Database integration (PostgreSQL/SQLite)
- [ ] Input validation and sanitization
- [ ] Authentication and authorization
- [ ] Pagination for large datasets
- [ ] Comprehensive unit tests
- [ ] Docker containerization
- [ ] API documentation with OpenAPI/Swagger

## 🤝 Contributing

This is a learning project, but contributions and suggestions are welcome! Feel free to:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📝 License

This project is open source and available under the [MIT License](LICENSE).

## 🎓 Acknowledgments

Built as part of learning Rust web development. Special thanks to the Rust community and the maintainers of Axum, Tokio, and Serde for their excellent documentation and tools.

---

**Happy coding! 🦀**
