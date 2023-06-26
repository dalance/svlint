
@echo off
for /f %%E in ('where.exe svls-designintent') do (
    set "SVLINT_CONFIG=%%~dpEdesignintent.toml"
)
svls %*

