
@echo off
for /f %%E in ('where.exe svlint-DaveMcEwan-design') do (
    set "SVLINT_CONFIG=%%~dpEDaveMcEwan-design.toml"
)
svlint %*

