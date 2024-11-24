#!/bin/bash

# Build the Docker image
docker build -t rust-webapp-deploy .

# Stop and remove existing container if it exists
docker stop deploy-test 2>/dev/null
docker rm deploy-test 2>/dev/null

# Run the container
docker run -d --name deploy-test -p 2222:22 -p 3000:3000 rust-webapp-deploy

# Wait for SSH to be ready
sleep 5

# Add the public key to the container
PUBLIC_KEY=$(cat github_deploy_key.pub)
docker exec deploy-test /bin/bash -c "echo '$PUBLIC_KEY' > /home/deploy/.ssh/authorized_keys"
docker exec deploy-test chown -R deploy:deploy /home/deploy/.ssh
docker exec deploy-test chmod 600 /home/deploy/.ssh/authorized_keys

# Get container IP
CONTAINER_IP=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' deploy-test)

echo -e "\nLocal deployment environment is ready!"
echo "Use these values for GitHub secrets:"
echo "PROD_SERVER_IP: localhost"
echo "PROD_USER: deploy"
echo -e "\nThe container is accessible via SSH on port 2222"
echo "Test SSH connection with: ssh -p 2222 -i github_deploy_key deploy@localhost"
