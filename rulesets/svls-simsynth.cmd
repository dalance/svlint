
@echo off
for /f %%E in ('where.exe svls-simsynth') do (
    set "SVLINT_CONFIG=%%~dpEsimsynth.toml"
)
svls %*

