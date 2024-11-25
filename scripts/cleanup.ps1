# Cleanup script for rust-webapp project

$scriptPath = $PSScriptRoot
Import-Module "$scriptPath\modules\Config.psm1"

# Get configuration
$config = Get-ProjectConfig
$cleanupConfig = $config.cleanup

Write-Host "Cleaning up build artifacts and temporary files..."

# Clean Rust build artifacts
if (Test-CommandAvailable "cargo") {
    cargo clean
    Write-Host " Cleaned Rust build artifacts"
}

# Clean frontend build artifacts
if ($cleanupConfig.clean_frontend) {
    $frontendPaths = @("frontend/node_modules", "frontend/dist")
    foreach ($path in $frontendPaths) {
        if (Test-Path $path) {
            Remove-Item -Path $path -Recurse -Force
            Write-Host " Cleaned $path"
        }
    }
}

# Clean Docker cache
if ($cleanupConfig.clean_docker_cache -and (Test-CommandAvailable "docker")) {
    docker builder prune -f
    Write-Host " Cleaned Docker build cache"
}

# Remove temporary files but preserve specified patterns
$cleanPatterns = $cleanupConfig.clean_patterns
$preservePatterns = $cleanupConfig.preserve_patterns

foreach ($pattern in $cleanPatterns) {
    $filesToRemove = Get-ChildItem -Path . -Include $pattern -Recurse |
        Where-Object {
            $file = $_
            -not ($preservePatterns | Where-Object { $file.Name -like $_ })
        }
    
    if ($filesToRemove) {
        $filesToRemove | Remove-Item -Force
        Write-Host " Removed temporary files matching: $pattern"
    }
}

Write-Host "`nCleanup complete!"
