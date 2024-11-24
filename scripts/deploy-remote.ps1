# Stop and remove existing containers
echo "Stopping existing containers..."
wsl docker compose down

# Start the containers using Docker Compose
echo "Starting containers..."
wsl docker compose up -d

echo "Deployment complete!"
echo "Your application should now be accessible at:"
echo "- http://quizmo.me (no port number needed)"
echo ""
echo "Next steps:"
echo "1. Configure port forwarding on your router (192.168.1.1):"
echo "   - External Port: 80"
echo "   - Internal IP: 192.168.1.172"
echo "   - Internal Port: 80"
echo "   - Protocol: TCP"
