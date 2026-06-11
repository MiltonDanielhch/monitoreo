$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$envFile = Join-Path $scriptPath "..\.env.local"

$dbUrl = Get-Content $envFile | Select-String "DATABASE_URL" | ForEach-Object { $_.ToString().Split("=")[1] }

if ($dbUrl -match "mysql://([^:]+):([^@]+)@([^:]+):(\d+)/(.+)") {
    $user = $matches[1]
    $pass = $matches[2]
    $dbHost = $matches[3]
    $port = $matches[4]
    $dbName = $matches[5]
    
    Write-Host "Ejecutando migraciones..."
    $migrationsPath = Join-Path $scriptPath "..\data\migrations"
    Get-ChildItem $migrationsPath\*.sql | Sort-Object Name | ForEach-Object {
        Write-Host "  - $($_.Name)"
        $content = Get-Content $_.FullName -Raw
        mysql -h $dbHost -P $port -u $user -p$pass $dbName -e $content 2>$null
    }
    
    Write-Host "Ejecutando seeds de sistema..."
    $seedsPath = Join-Path $scriptPath "..\data\seeds"
    Get-ChildItem $seedsPath\*.sql | Sort-Object Name | ForEach-Object {
        Write-Host "  - $($_.Name)"
        $content = Get-Content $_.FullName -Raw
        mysql -h $dbHost -P $port -u $user -p$pass $dbName -e $content 2>$null
    }
    
    Write-Host "Migraciones y seeds completadas"
} else {
    Write-Host "Error: No se pudo parsear DATABASE_URL"
}
