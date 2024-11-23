#!/bin/bash

echo "=== Rust Web Application Deployment ==="

# Build the release version
echo "Building release version..."
cargo build --release

echo """
Deployment built successfully! To complete the deployment, please run the following commands with sudo:

1. Install Nginx if not already installed:
   sudo apt update
   sudo apt install nginx

2. Copy Nginx configuration:
   sudo cp nginx.conf /etc/nginx/sites-available/quizmo.me
   sudo ln -s /etc/nginx/sites-available/quizmo.me /etc/nginx/sites-enabled/
   sudo rm /etc/nginx/sites-enabled/default  # Remove default config
   sudo nginx -t  # Test configuration
   sudo systemctl restart nginx

3. Install certbot for SSL:
   sudo apt install certbot python3-certbot-nginx
   sudo certbot --nginx -d quizmo.me -d www.quizmo.me

4. Copy and enable systemd service:
   sudo cp quizmo-web.service /etc/systemd/system/
   sudo systemctl daemon-reload
   sudo systemctl enable quizmo-web
   sudo systemctl start quizmo-web
   sudo systemctl status quizmo-web

After running these commands, your application should be accessible at https://quizmo.me
"""
