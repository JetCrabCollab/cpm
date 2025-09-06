# CPM Docker Build Script for Windows
Write-Host "🦀 Building CPM with Docker..." -ForegroundColor Green

# Build the Docker image
docker build -t cpm:latest .

if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ CPM Docker image built successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Usage examples:" -ForegroundColor Yellow
    Write-Host "  docker run --rm cpm --help"
    Write-Host "  docker run --rm -v `$(pwd):/workspace cpm init my-project -y"
    Write-Host "  docker-compose up cpm-test"
    Write-Host "  docker-compose up cpm-clippy"
} else {
    Write-Host "❌ Docker build failed!" -ForegroundColor Red
    exit 1
}
