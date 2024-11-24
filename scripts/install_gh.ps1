# Download GitHub CLI
$url = "https://github.com/cli/cli/releases/download/v2.37.0/gh_2.37.0_windows_amd64.msi"
$output = "gh_installer.msi"
Invoke-WebRequest -Uri $url -OutFile $output

# Install GitHub CLI
Start-Process msiexec.exe -Wait -ArgumentList "/I gh_installer.msi /quiet"

# Remove installer
Remove-Item gh_installer.msi

# Test installation
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
gh --version

Write-Host "`nGitHub CLI has been installed! Now you can run setup_secrets.ps1 to configure your repository."
