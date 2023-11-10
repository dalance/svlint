
@echo off
for /f %%E in ('where.exe svls-designintentV2001') do (
    set "SVLINT_CONFIG=%%~dpEdesignintentV2001.toml"
)
svls %*

