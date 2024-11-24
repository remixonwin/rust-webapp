$ErrorActionPreference = "Stop"

Write-Host "=== Testing HTTP Methods ==="

$baseUrl = "http://127.0.0.1:3000"
$tests = @(
    @{
        Endpoint = "/health"
        Method = "GET"
        ExpectedSuccess = $true
    },
    @{
        Endpoint = "/health"
        Method = "POST"
        ExpectedSuccess = $false
    },
    @{
        Endpoint = "/hello"
        Method = "GET"
        ExpectedSuccess = $true
    },
    @{
        Endpoint = "/hello"
        Method = "PUT"
        ExpectedSuccess = $false
    }
)

$totalTests = $tests.Count
$passed = 0

Write-Host "`nTesting $totalTests method combinations..."

foreach ($test in $tests) {
    $url = $baseUrl + $test.Endpoint
    try {
        $response = Invoke-RestMethod -Uri $url -Method $test.Method
        if ($test.ExpectedSuccess) {
            Write-Host "[PASS] $($test.Method) $url" -ForegroundColor Green
            $passed++
        }
        else {
            Write-Host "[FAIL] $($test.Method) $url - Expected failure but got success" -ForegroundColor Red
        }
    }
    catch {
        if (-not $test.ExpectedSuccess) {
            Write-Host "[PASS] $($test.Method) $url - Expected failure" -ForegroundColor Green
            $passed++
        }
        else {
            Write-Host "[FAIL] $($test.Method) $url - $($_.Exception.Message)" -ForegroundColor Red
        }
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
