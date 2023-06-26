
@echo off
for /f %%E in ('where.exe svls-DaveMcEwan-design') do (
    set "SVLINT_CONFIG=%%~dpEDaveMcEwan-design.toml"
)
svls %*

