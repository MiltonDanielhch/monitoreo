# Script para ver estado de migraciones aplicadas
# Vinculado con ADR-0005 (Migraciones y Seeding)

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$envFile = Join-Path $scriptPath "..\.env.local"

$dbUrl = (Get-Content $envFile | Select-String "DATABASE_URL" | ForEach-Object { $_.ToString().Split("=")[1] })

if ($dbUrl -match "mysql://([^:]+):([^@]+)@([^:]+):(\d+)/(.+)") {
    $user = $matches[1]
    $pass = $matches[2]
    $dbHost = $matches[3]
    $port = $matches[4]
    $db = $matches[5]
    
    Write-Host "Tablas en la base de datos:"
    mysql -h $dbHost -P $port -u $user -p$pass $db -e "SHOW TABLES;" 2>$null
} else {
    Write-Host "Error: No se pudo parsear DATABASE_URL"
}
