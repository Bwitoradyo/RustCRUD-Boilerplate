# Rust CRUD Boilerplate

A REST API boilerplate built with Rust, Rocket framework, and MongoDB for user management operations.

## Features

- ✅ Full CRUD operations (Create, Read, Update, Delete)
- 🚀 Built with Rocket web framework
- �️ MongoDB database integration
- 📝 JSON API endpoints
- 🆔 ObjectId-based user identification
- 🔧 Environment-based configuration

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/users` | Get all users |
| GET | `/api/users/{id}` | Get user by ObjectId |
| POST | `/api/users` | Create new user |
| PUT | `/api/users/{id}` | Update user |
| DELETE | `/api/users/{id}` | Delete user |

## Quick Start

### Prerequisites
- Rust (latest stable)
- MongoDB running on localhost:27017

1. **Clone the repository**
   ```bash
   git clone https://github.com/Bwitoradyo/RustCRUD-Boilerplate.git
   cd RustCRUD-Boilerplate
   ```

2. **Setup environment**
   ```bash
   cp .env.example .env
   # Edit .env if needed for custom MongoDB settings
   ```

3. **Run the application**
   ```bash
   cargo run
   ```

4. **Test the API**
   ```bash
   # Get all users
   curl http://127.0.0.1:8000/api/users
   
   # Create a user
   curl -X POST http://127.0.0.1:8000/api/users \
     -H "Content-Type: application/json" \
     -d '{"name": "John Doe", "email": "john@example.com"}'
   
   # Get user by ID (use ObjectId from create response)
   curl http://127.0.0.1:8000/api/users/507f1f77bcf86cd799439011
   ```

## Configuration

Environment variables (`.env` file):

```bash
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=rocketboiler
```

## Project Structure

```
src/
├── main.rs      # Application entry point & server setup
├── handlers.rs  # API route handlers (CRUD operations)
├── models.rs    # Data models (User, CreateUser, UpdateUser)
└── state.rs     # Database connection management
```

## Dependencies

- **Rocket** - Async web framework
- **MongoDB** - Official MongoDB driver
- **Serde** - JSON serialization/deserialization
- **BSON** - MongoDB document format support
- **Dotenv** - Environment variable management

## Response Format

### User Object
```json
{
  "id": "507f1f77bcf86cd799439011",
  "name": "John Doe", 
  "email": "john@example.com"
}

## License

MIT License
