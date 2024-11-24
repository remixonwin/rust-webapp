# Cleanup script for rust-webapp project

# Remove installation files
Remove-Item -Path "DockerDesktopInstaller.exe" -Force
Remove-Item -Path "rustup-init.exe" -Force
Remove-Item -Path "rustup-init.sh" -Force

# Remove quizmo-related files
Remove-Item -Path "quizmo-web.service" -Force
Remove-Item -Path "quizmo.me" -Force
Remove-Item -Path "quizmo.service" -Force

# Remove duplicate shell scripts (keeping PowerShell versions)
Remove-Item -Path "setup_local_deploy.sh" -Force
Remove-Item -Path "deploy.sh" -Force
Remove-Item -Path "setup_server.sh" -Force

# Remove sensitive files
Remove-Item -Path "github_deploy_key" -Force
Remove-Item -Path "github_deploy_key.pub" -Force

Write-Host "Cleanup completed successfully!"
