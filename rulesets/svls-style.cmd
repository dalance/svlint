
@echo off
for /f %%E in ('where.exe svls-style') do (
    set "SVLINT_CONFIG=%%~dpEstyle.toml"
)
svls %*

