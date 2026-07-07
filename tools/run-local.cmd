@echo off
setlocal
where pwsh >nul 2>nul
if %errorlevel%==0 (
  set "WOW_PS=pwsh"
) else (
  set "WOW_PS=powershell"
)
start "WoW Management Server Tray" "%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-server.ps1" -Tray
timeout /t 4 /nobreak >nul
start "WoW Client Agent Tray" "%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-client.ps1" -Tray
echo.
echo Web Admin: http://127.0.0.1:18080
echo Server tray and Client tray will keep running. Use tray menus to control server, monitor, service and settings.
pause
