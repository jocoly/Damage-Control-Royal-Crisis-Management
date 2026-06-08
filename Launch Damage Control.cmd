@echo off
setlocal
cd /d "%~dp0"

echo Starting Damage Control in development mode...
call pnpm desktop:dev
if errorlevel 1 (
  echo.
  echo Development launch failed.
  pause
  exit /b 1
)
