# Rust CRUD Boilerplate

A simple REST API boilerplate built with Rust and Rocket framework for user management operations.

## Features

- ✅ Full CRUD operations (Create, Read, Update, Delete)
- 🚀 Built with Rocket web framework
- 🔒 Thread-safe in-memory storage
- 📝 JSON API endpoints
- 🆔 UUID-based user identification

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | Get all users |
| GET | `/api/users/{id}` | Get user by ID |
| POST | `/api/users` | Create new user |
| PUT | `/api/users/{id}` | Update user |
| DELETE | `/api/users/{id}` | Delete user |

## Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/Bwitoradyo/RustCRUD-Bilerplate.git
   cd RustCRUD-Bilerplate
   ```

2. **Run the application**
   ```bash
   cargo run
   ```

3. **Test the API**
   ```bash
   # Get all users
   curl http://127.0.0.1:8000/api/users
   
   # Create a user
   curl -X POST http://127.0.0.1:8000/api/users \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "email": "john@example.com"}'
   ```

## Project Structure

```
src/
├── main.rs      # Application entry point
├── handlers.rs  # API route handlers
├── models.rs    # Data models
└── state.rs     # Application state management
```

## Dependencies

- **Rocket** - Web framework
- **Serde** - Serialization/deserialization
- **UUID** - Unique identifier generation

## License

MIT License
