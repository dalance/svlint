
@echo off
for /f %f in ('where.exe style') do set "WHERE=%f"
set "SVLINT_CONFIG=%WHERE%\style.toml"
svlint %*

