# Script para resetear base de datos local (peligroso - solo desarrollo)
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
    
    $confirm = Read-Host "¿Eliminar TODOS los datos de la base de datos local? (escriba SI para confirmar)"
    if ($confirm -eq "SI") {
        Write-Host "Eliminando tablas..."
        mysql -h $dbHost -P $port -u $user -p$pass $db -e "DROP TABLE IF EXISTS user_sessions, users, roles;" 2>$null
        Write-Host "Re-ejecutando migraciones..."
        cd $scriptPath; just db-migrate
    } else {
        Write-Host "Operación cancelada"
    }
} else {
    Write-Host "Error: No se pudo parsear DATABASE_URL"
}
