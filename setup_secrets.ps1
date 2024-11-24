# Get the repository name from git config
$repoUrl = git config --get remote.origin.url
$repoName = $repoUrl -replace '.*github\.com[:/](.*)\.git$','$1'

Write-Host "Setting up secrets for repository: $repoName"

# Read the deploy key
$deployKey = Get-Content "github_deploy_key" -Raw

# Find GitHub CLI
$ghPaths = @(
    "C:\Program Files\GitHub CLI\gh.exe",
    "C:\Program Files (x86)\GitHub CLI\gh.exe",
    "${env:LOCALAPPDATA}\GitHub CLI\gh.exe"
)

$ghPath = $ghPaths | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $ghPath) {
    Write-Error "GitHub CLI not found. Please ensure it's installed correctly."
    exit 1
}

Write-Host "Found GitHub CLI at: $ghPath"

# Get GitHub token
$token = Read-Host -Prompt "Enter your GitHub token with repo and admin permissions"
$env:GITHUB_TOKEN = $token

# Check auth status
& $ghPath auth status
if ($LASTEXITCODE -ne 0) {
    Write-Host "Please login to GitHub CLI first using: gh auth login"
    & $ghPath auth login
}

# Set the secrets
Write-Host "Setting DEPLOY_HOST..."
echo "quizmo.me" | & $ghPath secret set DEPLOY_HOST

Write-Host "Setting DEPLOY_USER..."
echo "remixonwin" | & $ghPath secret set DEPLOY_USER

Write-Host "Setting DEPLOY_SSH_KEY..."
$deployKey | & $ghPath secret set DEPLOY_SSH_KEY

# Prompt for email
$email = Read-Host -Prompt "Enter your email for SSL certificate notifications"
echo $email | & $ghPath secret set CERTBOT_EMAIL

Write-Host "`nSecrets have been set successfully!"
Write-Host "Next steps:"
Write-Host "1. Make sure the deploy key is added to your server at ~/.ssh/authorized_keys"
Write-Host "2. Create and push a tag to trigger the deployment:"
Write-Host "   git tag -a v1.0.0 -m 'Initial release'"
Write-Host "   git push origin v1.0.0"
