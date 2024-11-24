# Stop and remove existing container
echo "Stopping existing container..."
wsl bash -c 'docker stop rust-webapp-container 2>/dev/null || true'
wsl bash -c 'docker rm rust-webapp-container 2>/dev/null || true'

# Create network if it doesn't exist
echo "Setting up Docker network..."
wsl bash -c 'docker network inspect rust-webapp-net >/dev/null 2>&1 || docker network create rust-webapp-net'

# Run new container
echo "Starting new container..."
wsl docker run -d `
    --name rust-webapp-container `
    --restart always `
    --network rust-webapp-net `
    -p 3000:3000 `
    -e RUST_LOG=info `
    rust-webapp

echo "Deployment complete!"
