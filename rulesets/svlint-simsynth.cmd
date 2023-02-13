
@echo off
for /f %%E in ('where.exe svlint-simsynth') do (
    set "SVLINT_CONFIG=%%~dpEsimsynth.toml"
)
svlint %*

