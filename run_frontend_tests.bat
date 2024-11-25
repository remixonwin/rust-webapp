@echo off
setlocal

rem Set test configuration
set TEST_APP_URL=http://localhost:8081
set TEST_WEBDRIVER_URL=http://localhost:9515
set TEST_IMPLICIT_WAIT_TIMEOUT=5
set TEST_PAGE_LOAD_TIMEOUT=10
set TEST_SCRIPT_TIMEOUT=5

echo Starting ChromeDriver...
start /B tools\chromedriver.exe --port=9515

echo Waiting for ChromeDriver to start...
timeout /t 2 /nobreak >nul

echo Starting web server...
start /B cargo run

echo Waiting for web server to start...
timeout /t 5 /nobreak >nul

echo Running frontend tests...
cargo test --test mod frontend -- --nocapture --test-threads=1

echo Cleaning up...
taskkill /F /IM chromedriver.exe >nul 2>&1
taskkill /F /IM rust-webapp.exe >nul 2>&1

echo Done!
endlocal
