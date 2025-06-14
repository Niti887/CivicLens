# CivicLens

A community-driven issue reporting and resolution platform built with Rust.

## Features

- AI-Powered Issue Detection
- Real-Time Geolocation & Mapping
- Gamification & Community Engagement
- Automated Escalation & Tracking
- Crowdsourced Verification
- Integration with Local Government Systems
- Public Dashboard & Analytics

## Project Structure

```
civiclens/
├── backend/         # Actix-web backend server
└── frontend/        # Yew frontend application
```

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- wasm-pack (for frontend)
- PostgreSQL (for database)

## Setup

1. Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install wasm-pack:
```bash
cargo install wasm-pack
```

3. Install PostgreSQL and create a database:
```bash
createdb civiclens
```

## Running the Application

### Backend

1. Navigate to the backend directory:
```bash
cd backend
```

2. Run the server:
```bash
cargo run
```

The backend server will start at http://localhost:8080

### Frontend

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Build the frontend:
```bash
wasm-pack build --target web
```

3. Serve the frontend (using a simple HTTP server):
```bash
python3 -m http.server 8000
```

The frontend will be available at http://localhost:8000

## API Endpoints

- `POST /api/issues` - Report a new issue
- `GET /api/issues` - Get all issues
- `GET /api/issues/{id}` - Get a specific issue
- `PUT /api/issues/{id}/status/{status}` - Update issue status

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
