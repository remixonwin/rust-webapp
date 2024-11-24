# Test Changes PowerShell Script
Write-Host "=== Testing Rust WebApp Changes ==="

$endpoints = @(
    "http://127.0.0.1:3000/",
    "http://127.0.0.1:3000/static/index.html",
    "http://127.0.0.1:3000/health",
    "http://127.0.0.1:3000/hello"
)

Write-Host "Testing endpoints..."
$success = 0

foreach ($endpoint in $endpoints) {
    try {
        $response = Invoke-WebRequest -Uri $endpoint -Method GET -ErrorAction Stop
        Write-Host "✓ GET $endpoint - Success (Status: $($response.StatusCode))"
        $success++
    } catch {
        Write-Host "✗ GET $endpoint - Failed: $_"
    }
}

Write-Host "`nTest Summary:"
Write-Host "Passed: $success/$($endpoints.Count) tests"
