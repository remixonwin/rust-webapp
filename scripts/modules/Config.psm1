# Configuration module for rust-webapp scripts

function Get-ProjectRoot {
    $scriptPath = $PSScriptRoot
    return (Get-Item $scriptPath).Parent.Parent.FullName
}

function Get-ConfigPath {
    $projectPath = Get-ProjectRoot
    return Join-Path -Path $projectPath -ChildPath "config\scripts.config.toml"
}

function Test-WSL {
    $wsl = Get-Command wsl -ErrorAction SilentlyContinue
    return $null -ne $wsl
}

function Test-DockerWSL {
    if (-not (Test-WSL)) {
        Write-Error "WSL is not installed. Please install WSL and try again."
        return $false
    }
    
    $dockerCheck = wsl docker --version 2>$null
    return $LASTEXITCODE -eq 0
}

function Invoke-DockerCommand {
    param (
        [Parameter(Mandatory=$true)]
        [string]$Command,
        
        [Parameter(Mandatory=$false)]
        [string]$WorkingDirectory = ""
    )
    
    if (-not (Test-DockerWSL)) {
        return $false
    }

    if ($WorkingDirectory) {
        # Convert Windows path to WSL path
        $wslPath = wsl wslpath -u "'$WorkingDirectory'" 2>$null
        if ($LASTEXITCODE -eq 0) {
            $Command = "cd $wslPath && $Command"
        }
        else {
            Write-Error "Failed to convert Windows path to WSL path"
            return $false
        }
    }
    
    $result = wsl bash -c "$Command"
    return $result
}

function ConvertTo-WSLPath {
    param (
        [Parameter(Mandatory=$true)]
        [string]$WindowsPath
    )
    
    $wslPath = wsl wslpath -u "'$WindowsPath'" 2>$null
    if ($LASTEXITCODE -eq 0) {
        return $wslPath.Trim("'")
    }
    return $null
}

function ConvertFrom-Toml {
    param (
        [string]$TomlContent
    )
    
    $config = @{}
    $currentSection = $config
    $sectionStack = @()
    
    foreach ($line in $TomlContent -split "`n") {
        $line = $line.Trim()
        if (-not $line -or $line.StartsWith("#")) { continue }
        
        if ($line -match '^\[(.*)\]$') {
            $section = $matches[1]
            $currentSection = $config
            foreach ($part in $section -split '\.') {
                if (-not $currentSection.ContainsKey($part)) {
                    $currentSection[$part] = @{}
                }
                $currentSection = $currentSection[$part]
            }
        }
        elseif ($line -match '^([^=]+)=(.*)$') {
            $key = $matches[1].Trim()
            $value = $matches[2].Trim()
            
            # Handle arrays
            if ($value.StartsWith("[") -and $value.EndsWith("]")) {
                $array = $value.Substring(1, $value.Length - 2) -split ","
                $currentSection[$key] = $array | ForEach-Object { $_.Trim(' "''') }
            }
            # Handle strings
            elseif ($value.StartsWith('"') -or $value.StartsWith("'")) {
                $currentSection[$key] = $value.Substring(1, $value.Length - 2)
            }
            # Handle numbers and booleans
            else {
                switch -regex ($value) {
                    '^-?\d+$' { $currentSection[$key] = [int]$value; break }
                    '^-?\d*\.\d+$' { $currentSection[$key] = [double]$value; break }
                    '^(true|false)$' { $currentSection[$key] = [bool]::Parse($value); break }
                    default { $currentSection[$key] = $value }
                }
            }
        }
    }
    
    return $config
}

function Get-ProjectConfig {
    $configPath = Get-ConfigPath
    Write-Debug "Config path: $configPath"
    
    if (-not (Test-Path -Path $configPath)) {
        Write-Error "Configuration file not found at: $configPath"
        return @{}
    }
    
    try {
        $tomlContent = Get-Content -Path $configPath -Raw -ErrorAction Stop
        return ConvertFrom-Toml -TomlContent $tomlContent
    }
    catch {
        Write-Error "Error reading configuration: $_"
        return @{}
    }
}

function Test-AdminPrivileges {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Test-CommandAvailable {
    param (
        [string]$Command
    )
    
    return [bool](Get-Command -Name $Command -ErrorAction SilentlyContinue)
}

Export-ModuleMember -Function @(
    'Get-ProjectRoot',
    'Get-ProjectConfig',
    'Test-AdminPrivileges',
    'Test-CommandAvailable',
    'Test-DockerWSL',
    'Invoke-DockerCommand',
    'ConvertTo-WSLPath'
)
