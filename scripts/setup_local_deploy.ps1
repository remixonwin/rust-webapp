# Build the Docker image
docker build -t rust-webapp-deploy .

# Stop and remove existing container if it exists
docker stop deploy-test 2>$null
docker rm deploy-test 2>$null

# Run the container
docker run -d --name deploy-test -p 2222:22 -p 3000:3000 rust-webapp-deploy

# Wait for SSH to be ready
Start-Sleep -Seconds 5

# Add the public key to the container
$publicKey = Get-Content .\github_deploy_key.pub
docker exec deploy-test /bin/bash -c "echo '$publicKey' > /home/deploy/.ssh/authorized_keys"
docker exec deploy-test chown -R deploy:deploy /home/deploy/.ssh
docker exec deploy-test chmod 600 /home/deploy/.ssh/authorized_keys

# Get container IP
$containerIP = docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' deploy-test

Write-Host "`nLocal deployment environment is ready!"
Write-Host "Use these values for GitHub secrets:"
Write-Host "PROD_SERVER_IP: localhost"
Write-Host "PROD_USER: deploy"
Write-Host "`nThe container is accessible via SSH on port 2222"
Write-Host "Test SSH connection with: ssh -p 2222 -i github_deploy_key deploy@localhost"
