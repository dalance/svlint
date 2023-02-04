
@echo off
for /f %%E in ('where.exe svlint-verifintent') do (
    set "SVLINT_CONFIG=%%~dpEverifintent.toml"
)
svlint %*

