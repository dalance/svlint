
@echo off
for /f %%E in ('where.exe svlint-parseonly') do (
    set "SVLINT_CONFIG=%%~dpEparseonly.toml"
)
svlint %*

