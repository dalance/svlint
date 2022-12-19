
@echo off
for /f %f in ('where.exe parseonly') do set "WHERE=%f"
set "SVLINT_CONFIG=%WHERE%\parseonly.toml"
svlint %*

