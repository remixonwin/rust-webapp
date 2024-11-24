$ErrorActionPreference = "Stop"

Write-Host "=== Testing Content Types ==="

$baseUrl = "http://127.0.0.1:3000"
$tests = @(
    @{
        Endpoint = "/"
        ExpectedContentType = "text/html"
    },
    @{
        Endpoint = "/health"
        ExpectedContentType = "application/json"
    },
    @{
        Endpoint = "/hello"
        ExpectedContentType = "application/json"
    }
)

$totalTests = $tests.Count
$passed = 0

Write-Host "`nTesting $totalTests content types..."

foreach ($test in $tests) {
    $url = $baseUrl + $test.Endpoint
    try {
        $response = Invoke-WebRequest -Uri $url -Method Get
        $contentType = $response.Headers["Content-Type"]
        if ($contentType -like "*$($test.ExpectedContentType)*") {
            Write-Host "[PASS] $url - Content-Type: $contentType" -ForegroundColor Green
            $passed++
        }
        else {
            Write-Host "[FAIL] $url - Expected $($test.ExpectedContentType), got $contentType" -ForegroundColor Red
        }
    }
    catch {
        Write-Host "[FAIL] $url - $($_.Exception.Message)" -ForegroundColor Red
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
