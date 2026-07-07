@echo off
setlocal
where pwsh >nul 2>nul
if %errorlevel%==0 (
  set "WOW_PS=pwsh"
) else (
  set "WOW_PS=powershell"
)
start "WoW Management Server" "%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-server.ps1" -OpenBrowser
timeout /t 4 /nobreak >nul
start "WoW Client Agent" "%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-client.ps1" -Monitor
echo.
echo Web Admin: http://127.0.0.1:18080
echo Server and Client monitor windows will keep running. Close those windows to stop them.
pause
