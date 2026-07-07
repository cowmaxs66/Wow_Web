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
start "WoW Client Agent Tray" "%WOW_PS%" -NoProfile -ExecutionPolicy Bypass -File "%~dp0start-client.ps1" -Tray
echo.
echo Web Admin: http://127.0.0.1:18080
echo Server window and Client tray will keep running. Use the tray menu to control monitor/service/settings.
pause
