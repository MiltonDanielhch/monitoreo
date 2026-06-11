# Script para ejecutar migraciones y seeds en producción
# Vinculado con ADR-0005 (Migraciones y Seeding)

$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$envFile = Join-Path $scriptPath "..\.env"

$dbUrl = (Get-Content $envFile | Select-String "DATABASE_URL" | ForEach-Object { $_.ToString().Split("=")[1] })

if ($dbUrl -match "mysql://([^:]+):([^@]+)@([^:]+):(\d+)/(.+)") {
    $user = $matches[1]
    $pass = $matches[2]
    $dbHost = $matches[3]
    $port = $matches[4]
    $db = $matches[5]
    
    $confirm = Read-Host "¿Ejecutar migraciones en PRODUCCIÓN? (escriba SI para confirmar)"
    if ($confirm -eq "SI") {
        Write-Host "Ejecutando migraciones..."
        $migrationsPath = Join-Path $scriptPath "..\data\migrations"
        Get-ChildItem $migrationsPath\*.sql | Sort-Object Name | ForEach-Object {
            Write-Host "  - $($_.Name)"
            $content = Get-Content $_.FullName -Raw
            mysql -h $dbHost -P $port -u $user -p$pass $db -e $content
        }
        
        Write-Host "Ejecutando seeds de sistema..."
        $seedsPath = Join-Path $scriptPath "..\data\seeds"
        Get-ChildItem $seedsPath\*.sql | Sort-Object Name | ForEach-Object {
            Write-Host "  - $($_.Name)"
            $content = Get-Content $_.FullName -Raw
            mysql -h $dbHost -P $port -u $user -p$pass $db -e $content
        }
        
        Write-Host "✓ Migraciones y seeds de producción completadas"
    } else {
        Write-Host "Operación cancelada"
    }
} else {
    Write-Host "Error: No se pudo parsear DATABASE_URL"
}
