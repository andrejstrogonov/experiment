# build.ps1 - build the wasm frontend and prepare static assets

# navigate to the script directory
Push-Location -Path $PSScriptRoot

# run wasm-pack to compile in release mode
Write-Host "Running wasm-pack..." -ForegroundColor Cyan
wasm-pack build --target web --release
if ($LASTEXITCODE -ne 0) { throw "wasm-pack failed" }

# copy generated files into the static directory
Write-Host "Copying pkg outputs to static/..." -ForegroundColor Cyan
Copy-Item -Path pkg\* -Destination static -Force

Write-Host "Build complete. Serve web/static to run the app." -ForegroundColor Green

# restore previous directory
Pop-Location
