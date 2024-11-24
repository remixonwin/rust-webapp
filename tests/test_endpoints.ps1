# Test endpoints script
$ErrorActionPreference = "Stop"

Write-Host "=== Testing Rust WebApp Endpoints ===" -ForegroundColor Green

# Wait for server to be ready
Write-Host "Waiting for server to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 2

# Test endpoints
Write-Host "Testing endpoints..." -ForegroundColor Yellow

$baseUrl = "http://127.0.0.1:3000"
$endpoints = @("/", "/static/index.html", "/health", "/hello")
$totalTests = $endpoints.Count
$passed = 0

Write-Host "`nTesting $totalTests endpoints..."

foreach ($endpoint in $endpoints) {
    $url = $baseUrl + $endpoint
    try {
        $response = Invoke-RestMethod -Uri $url -Method Get
        Write-Host "[PASS] GET $url" -ForegroundColor Green
        $passed++
    }
    catch {
        Write-Host "[FAIL] GET $url - $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`nResults: $passed/$totalTests tests passed"
if ($passed -eq $totalTests) {
    Write-Host "All tests passed!" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "Some tests failed." -ForegroundColor Red
    exit 1
}

# Cleanup
Write-Host "Cleaning up..." -ForegroundColor Yellow
Get-Process -Name "rust-webapp" -ErrorAction SilentlyContinue | Stop-Process -Force
