# Local deployment script for rust-webapp

$scriptPath = $PSScriptRoot
Import-Module "$scriptPath\modules\Config.psm1"

# Get configuration
$config = Get-ProjectConfig
$deployConfig = $config.deployment
$projectRoot = Get-ProjectRoot
$dockerfilePath = Join-Path $projectRoot "docker"

# Check Docker availability through WSL
if (-not (Test-DockerWSL)) {
    Write-Host "Error: Docker is not available in WSL. Please ensure Docker is installed and running in WSL."
    exit 1
}

# Build the Docker image
Write-Host "Building Docker image..."
Invoke-DockerCommand -Command "docker build -t $($deployConfig.docker_image_name) -f docker/Dockerfile ." -WorkingDirectory $projectRoot

# Stop and remove existing container
Write-Host "Stopping existing container..."
Invoke-DockerCommand "docker ps -a -q -f 'name=$($deployConfig.container_name)' | xargs -r docker stop"
Invoke-DockerCommand "docker ps -a -q -f 'name=$($deployConfig.container_name)' | xargs -r docker rm"

# Create network if it doesn't exist
Write-Host "Setting up Docker network..."
Invoke-DockerCommand "docker network ls -q -f 'name=$($deployConfig.network_name)' || docker network create $($deployConfig.network_name)"

# Run new container
Write-Host "Starting new container..."
$dockerRunCmd = @(
    "docker run -d",
    "--name $($deployConfig.container_name)",
    "--restart always",
    "--network $($deployConfig.network_name)",
    "-p '$($deployConfig.default_port):$($deployConfig.default_port)'",
    "-e RUST_LOG=info",
    "$($deployConfig.docker_image_name)"
) -join " "

Invoke-DockerCommand $dockerRunCmd

Write-Host "`nDeployment complete!"
Write-Host "Application is now running at http://localhost:$($deployConfig.default_port)"
Write-Host "API documentation available at http://localhost:$($deployConfig.default_port)$($config.development.api_docs_path)"
