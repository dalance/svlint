
@echo off
for /f %%E in ('where.exe svls-verifintent') do (
    set "SVLINT_CONFIG=%%~dpEverifintent.toml"
)
svls %*

