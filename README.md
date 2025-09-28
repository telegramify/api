# Telegramify API

A modern RESTful API built with Rust, featuring PostgreSQL, Redis, Telegram authentication, file uploads, and comprehensive OpenAPI documentation.

## Features

- **🚀 High Performance**: Built with Axum web framework for maximum speed
- **🗄️ PostgreSQL**: Robust database integration with SQLx and migrations
- **⚡ Redis**: Fast caching and session management
- **🔐 Telegram Auth**: Secure authentication using Telegram's Login Widget
- **📤 File Uploads**: Multipart file upload support with size limits
- **📝 Form Processing**: JSON and form data handling
- **📖 OpenAPI/Swagger**: Auto-generated API documentation
- **🐳 Docker**: Complete containerization with docker-compose
- **🔧 Code Formatting**: Consistent code style with rustfmt

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Or: Rust 1.75+, PostgreSQL 16+, Redis 7+

### Using Docker (Recommended)

1. Clone the repository:
```bash
git clone <repository-url>
cd api
```

2. Copy and configure environment:
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. Start all services:
```bash
docker-compose up -d
```

4. The API will be available at:
   - API: http://localhost:3000
   - Swagger UI: http://localhost:3000/swagger-ui

### Manual Setup

1. Install dependencies and set up databases:
```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Start PostgreSQL and Redis (example with brew on macOS)
brew services start postgresql
brew services start redis

# Create database
createdb telegramify_api
```

2. Configure environment:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Run the application:
```bash
cargo run
```

## API Documentation

### Authentication

The API uses Telegram's Login Widget for authentication. To authenticate:

1. Implement Telegram Login Widget on your frontend
2. Send the authentication data to `/auth/telegram`
3. Use the returned JWT token in the `Authorization: Bearer <token>` header

### Endpoints

#### Health Check
- `GET /health` - Basic health check
- `GET /health/detailed` - Detailed health with database status

#### Authentication
- `POST /auth/telegram` - Authenticate with Telegram

#### Users
- `GET /users` - List all users
- `GET /users/{id}` - Get user by ID

#### Files
- `POST /upload` - Upload a file (multipart/form-data)
- `POST /form` - Process form data (JSON)

### Example Requests

#### Telegram Authentication
```bash
curl -X POST http://localhost:3000/auth/telegram \
  -H "Content-Type: application/json" \
  -d '{
    "id": 123456789,
    "first_name": "John",
    "last_name": "Doe",
    "username": "johndoe",
    "auth_date": 1640995200,
    "hash": "telegram_auth_hash_here"
  }'
```

#### File Upload
```bash
curl -X POST http://localhost:3000/upload \
  -F "file=@/path/to/your/file.jpg"
```

#### Form Data
```bash
curl -X POST http://localhost:3000/form \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com",
    "message": "Hello, world!"
  }'
```

## Configuration

All configuration is done through environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `REDIS_URL` | Redis connection string | Required |
| `SERVER_HOST` | Server bind address | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |
| `JWT_SECRET` | JWT signing secret | Required |
| `TELEGRAM_BOT_TOKEN` | Telegram bot token | Required |
| `MAX_FILE_SIZE` | Maximum file upload size (bytes) | `10485760` (10MB) |
| `UPLOAD_DIR` | File upload directory | `./uploads` |
| `RUST_LOG` | Logging level | `info` |

## Development

### Code Formatting

Format code with rustfmt:
```bash
cargo fmt
```

### Database Migrations

Create a new migration:
```bash
sqlx migrate add migration_name
```

Run migrations:
```bash
sqlx migrate run
```

### Testing

Run tests:
```bash
cargo test
```

## Architecture

```
src/
├── auth/           # Authentication modules
│   ├── jwt.rs      # JWT token handling
│   └── telegram.rs # Telegram auth verification
├── config/         # Configuration management
├── handlers/       # HTTP request handlers
│   ├── auth.rs     # Authentication endpoints
│   ├── files.rs    # File upload & form handling
│   ├── health.rs   # Health check endpoints
│   └── users.rs    # User management endpoints
├── models/         # Data models
├── utils/          # Utilities and error handling
├── lib.rs          # Library exports and app state
└── main.rs         # Application entry point
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt` and `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.