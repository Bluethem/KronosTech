# Script PowerShell para ejecutar el backend de KronosTech

Write-Host "KronosTech Backend - E-Commerce API" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host ""

# Verificar si Rust esta instalado
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Rust no esta instalado" -ForegroundColor Red
    Write-Host "   Instala Rust desde: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

Write-Host "Rust instalado correctamente" -ForegroundColor Green

# Verificar archivo .env
if (-not (Test-Path ".env")) {
    Write-Host "Advertencia: Archivo .env no encontrado" -ForegroundColor Yellow
    Write-Host "   Asegurate de configurar las variables de entorno" -ForegroundColor Yellow
} else {
    Write-Host "Archivo .env encontrado" -ForegroundColor Green
}

Write-Host ""
Write-Host "Compilando e iniciando servidor..." -ForegroundColor Cyan
Write-Host ""

# Ejecutar cargo
cargo run

# Si falla la ejecucion
if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "Error al iniciar el servidor" -ForegroundColor Red
    Write-Host ""
    Write-Host "Posibles soluciones:" -ForegroundColor Yellow
    Write-Host "  1. Verifica que PostgreSQL este corriendo" -ForegroundColor White
    Write-Host "  2. Verifica la configuracion en .env" -ForegroundColor White
    Write-Host "  3. Asegurate de que la base de datos kronosdb exista" -ForegroundColor White
    Write-Host "  4. Ejecuta los scripts SQL ddl.sql y dml.sql" -ForegroundColor White
    exit 1
}
