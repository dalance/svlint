
@echo off
for /f %%E in ('where.exe svlint-naming') do (
    set "SVLINT_CONFIG=%%~dpEnaming.toml"
)
svlint %*

