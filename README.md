# Rust Backend for rust peer experiments using SocialCalc/EtherCalc and libp2p

A modern Rust implementation of the SocialCalc (Aspiring Investments) backend using Axum framework.

## Features

- **Authentication**: JWT-based authentication with bcrypt password hashing
- **File Management**: PostgreSQL-based file storage with user isolation
- **Email Integration**: AWS SES integration for email notifications
- **Cloud Storage**: AWS S3 integration for file storage
- **In-App Purchases**: Purchase tracking and validation
- **RESTful API**: Clean REST API with JSON responses

## Tech Stack

- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT + bcrypt
- **Cloud Services**: AWS S3, AWS SES
- **Serialization**: Serde JSON
- **Async Runtime**: Tokio

## Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL
- AWS credentials (for S3 and SES)

### Environment Variables

```env
DATABASE_URL=postgresql://username:password@localhost/aspiring_investments
JWT_SECRET=your-jwt-secret
AWS_ACCESS_KEY_ID=your-aws-access-key
AWS_SECRET_ACCESS_KEY=your-aws-secret-key
AWS_REGION=us-east-1
S3_BUCKET=your-s3-bucket
SES_FROM_EMAIL=your-email@domain.com
```

### Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   cargo build
   ```
3. Run migrations:
   ```bash
   sqlx migrate run
   ```
4. Start the server:
   ```bash
   cargo run
   ```

The server will start on `http://0.0.0.0:8080`

## API Endpoints

### Authentication
- `POST /login` - User login
- `POST /register` - User registration  
- `GET /logout` - User logout
- `POST /lostpw` - Password reset request
- `POST /pwreset` - Password reset

### File Operations
- `GET /save` - List user files
- `POST /save` - Save file
- `POST /insert` - Get file content
- `POST /usersheet` - Handle user sheet operations

### Web App
- `GET /webapp` - Web app operations
- `POST /webapp` - Web app actions (save, delete, list files)

### Utilities
- `POST /runasemailer` - Send emails
- `GET /runas` - Run applications
- `POST /downloadfile` - Download files
- `GET/POST /htmltopdf` - HTML to PDF conversion
- `GET/POST /iconimg` - Image handling

## Database Schema

The application uses PostgreSQL with the following main tables:

- `users` - User accounts and authentication
- `files` - File storage with user isolation
- `in_app_purchases` - Purchase tracking

## Security Features

- Password hashing with bcrypt
- JWT token authentication
- User data isolation
- Input validation and sanitization
- CORS support

## Error Handling

The API returns consistent JSON responses:

```json
{
  "result": "ok|fail",
  "data": "response data",
  "message": "error message if applicable"
}
```

## Development

Run tests:
```bash
cargo test
```

Run with hot reload:
```bash
cargo watch -x run
```

## Production Deployment

1. Set all required environment variables
2. Run database migrations
3. Build release binary:
   ```bash
   cargo build --release
   ```
4. Deploy with proper reverse proxy (nginx recommended)
