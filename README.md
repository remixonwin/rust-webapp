# Quizmo Web Application

A Rust-based web application using Actix-web framework.

## Features

- Built with Actix-web 4.4
- HTTPS support with Let's Encrypt SSL
- Nginx reverse proxy configuration
- Systemd service integration
- JSON API endpoints

## Setup

### Prerequisites

- Rust (latest stable version)
- Nginx
- Certbot (for SSL)

### Installation

1. Clone the repository:
```bash
git clone [your-repo-url]
cd rust-webapp
```

2. Build the project:
```bash
cargo build --release
```

3. Set up Nginx configuration:
```bash
sudo cp nginx.conf /etc/nginx/sites-available/quizmo.me
sudo ln -s /etc/nginx/sites-available/quizmo.me /etc/nginx/sites-enabled/
```

4. Set up systemd service:
```bash
sudo cp quizmo-web.service /etc/systemd/system/
sudo systemctl enable quizmo-web
sudo systemctl start quizmo-web
```

## API Endpoints

- `GET /`: Welcome message
- `POST /echo`: Echoes back JSON message
- `GET /health`: Health check endpoint

## Development

```bash
cargo run
```

The application will start on `http://localhost:8080`

## Production Deployment

See `deploy.sh` for detailed deployment steps.

## License

MIT
