@echo off
setlocal
where pwsh >nul 2>nul
if %errorlevel%==0 (
  set "WOW_PS=pwsh"
) else (
  set "WOW_PS=powershell"
)
"%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-server.ps1" -OpenBrowser %*
