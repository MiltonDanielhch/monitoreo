comando para generar la clave secreta:
```bash
openssl rand -hex 32
```

```powershell
PS C:\laravel\monitoreo> $rng = [System.Security.Cryptography.RNGCryptoServiceProvider]::Create(); $bytes = New-Object byte[] 32; $rng.GetBytes($bytes); [System.BitConverter]::ToString($bytes).Replace("-","").ToLower()
d9545532af558d023cdf2ae150e68b8faf7417d3647a1cf32310c2b6b3b01280
```


PS C:\laravel\monitoreo> Get-Content data\migrations\0001_init_auth.sql | mysql -u root -pMilton123 -h 127.0.0.1 redes_dev
mysql: [Warning] Using a password on the command line interface can be insecure.



                         mysql -u root -pMilton123 -h 127.0.0.1 redes_dev -e "INSERT INTO roles (id, name) VALUES ('admin-role-id', 'ADMIN'), ('operator-role-id', 'OPERATOR'), ('monitor-role-id', 'MONITOR');"
mysql: [Warning] Using a password on the command line interface can be insecure.