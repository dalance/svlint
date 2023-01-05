
@echo off
for /f %%E in ('where.exe svlint-style') do (
    set "SVLINT_CONFIG=%%~dpEstyle.toml"
)
svlint %*

