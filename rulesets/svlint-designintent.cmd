
@echo off
for /f %%E in ('where.exe svlint-designintent') do (
    set "SVLINT_CONFIG=%%~dpEdesignintent.toml"
)
svlint %*

