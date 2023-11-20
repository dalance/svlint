
@echo off
for /f %%E in ('where.exe svlint-designintentV2001') do (
    set "SVLINT_CONFIG=%%~dpEdesignintentV2001.toml"
)
svlint %*

