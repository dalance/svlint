
@echo off
for /f %%E in ('where.exe svls-parseonly') do (
    set "SVLINT_CONFIG=%%~dpEparseonly.toml"
)
svls %*

