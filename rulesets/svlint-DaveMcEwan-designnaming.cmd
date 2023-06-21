
@echo off
for /f %%E in ('where.exe svlint-DaveMcEwan-designnaming') do (
    set "SVLINT_CONFIG=%%~dpEDaveMcEwan-designnaming.toml"
)
svlint %*

