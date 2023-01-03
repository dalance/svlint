
@echo off
for /f %f in ('where.exe naming') do set "WHERE=%f"
set "SVLINT_CONFIG=%WHERE%\naming.toml"
svlint %*

