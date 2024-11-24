#!/bin/bash

# Create .ssh directory if it doesn't exist
mkdir -p ~/.ssh
chmod 700 ~/.ssh

# Add the deploy key to authorized_keys
cat github_deploy_key.pub >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys

echo "Deploy key has been added to authorized_keys!"
echo "Testing the deployment key..."

# Test SSH connection
ssh -i github_deploy_key -o StrictHostKeyChecking=no remixonwin@quizmo.me "echo SSH connection successful!"
