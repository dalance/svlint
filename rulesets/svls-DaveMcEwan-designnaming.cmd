
@echo off
for /f %%E in ('where.exe svls-DaveMcEwan-designnaming') do (
    set "SVLINT_CONFIG=%%~dpEDaveMcEwan-designnaming.toml"
)
svls %*

