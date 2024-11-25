# Remote deployment script for rust-webapp

$scriptPath = $PSScriptRoot
Import-Module "$scriptPath\modules\Config.psm1"

# Get configuration
$config = Get-ProjectConfig
$remoteConfig = $config.remote
$projectRoot = Get-ProjectRoot
$dockerfilePath = Join-Path $projectRoot "docker"

# Ensure environment variables are set or use defaults
if (-not $env:DEPLOY_HOST) { $env:DEPLOY_HOST = $remoteConfig.default_host }
if (-not $env:DEPLOY_PORT) { $env:DEPLOY_PORT = $remoteConfig.default_port }

# Check Docker and Docker Compose availability through WSL
if (-not (Test-DockerWSL)) {
    Write-Host "Error: Docker is not available in WSL. Please ensure Docker is installed and running in WSL."
    exit 1
}

# Stop and remove existing containers
Write-Host "Stopping existing containers..."
Invoke-DockerCommand -Command "docker compose down" -WorkingDirectory $dockerfilePath

# Build and start the containers
Write-Host "Building and starting containers..."
Invoke-DockerCommand -Command "docker compose up -d --build" -WorkingDirectory $dockerfilePath

Write-Host "`nDeployment complete!"
Write-Host "Your application should now be accessible at:"
$protocol = if ($remoteConfig.ssl_enabled) { "https" } else { "http" }
$portDisplay = if ($env:DEPLOY_PORT -eq "80") { "" } else { ":$($env:DEPLOY_PORT)" }
Write-Host "$($protocol)://$($env:DEPLOY_HOST)$($portDisplay)"

Write-Host "`nNext steps:"
Write-Host "1. Ensure your firewall allows incoming traffic on port $($env:DEPLOY_PORT)"
if ($env:DEPLOY_HOST -ne "localhost") {
    Write-Host "2. Update your DNS settings to point to $($env:DEPLOY_HOST)"
}
if (-not $remoteConfig.ssl_enabled) {
    Write-Host "3. Consider setting up SSL/TLS for secure connections"
}
