@echo off
setlocal
cd /d "%~dp0"
title Immich Backup Manager - Rust Build

echo.
echo Immich Backup Manager - Rust Build
echo ===================================
echo.

where cargo.exe >nul 2>nul
if errorlevel 1 (
    echo FEHLER: Rust/Cargo wurde nicht gefunden.
    echo.
    echo Rust installieren: https://rustup.rs
    echo Danach dieses Fenster schliessen und BUILD.cmd erneut starten.
    echo.
    pause
    exit /b 1
)

echo [1/3] Rust-Version pruefen...
rustc --version
cargo --version
if errorlevel 1 goto :failed

echo.
echo [2/3] Quellcode pruefen...
cargo check
if errorlevel 1 goto :failed

echo.
echo [3/3] Optimierte Release-Version bauen...
cargo build --release
if errorlevel 1 goto :failed

copy /Y "target\release\immich_backup_manager.exe" "Immich Backup Manager.exe" >nul
if errorlevel 1 goto :failed

echo.
echo FERTIG:
echo %CD%\Immich Backup Manager.exe
echo.
pause
exit /b 0

:failed
echo.
echo FEHLER: Das Projekt konnte nicht erstellt werden.
echo Bitte den kompletten Fehlertext kopieren oder fotografieren.
echo.
pause
exit /b 1
