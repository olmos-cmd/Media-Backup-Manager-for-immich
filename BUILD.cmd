@echo off
setlocal
cd /d "%~dp0"
title Media Backup Manager - Rust Build

echo.
echo Media Backup Manager - Rust Build
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

echo [1/6] Rust-Version pruefen...
rustc --version
cargo --version
if errorlevel 1 goto :failed

echo.
echo [2/6] Cargo.lock pruefen...
if not exist "Cargo.lock" (
    echo FEHLER: Cargo.lock fehlt.
    echo Bitte zuerst "cargo generate-lockfile" ausfuehren und Cargo.lock in das Projekt aufnehmen.
    goto :failed
)

echo.
echo [3/6] Formatierung pruefen...
cargo fmt --check
if errorlevel 1 goto :failed

echo.
echo [4/6] Quellcode pruefen...
cargo check --locked
if errorlevel 1 goto :failed

echo.
echo [5/6] Tests ausfuehren...
cargo test --locked
if errorlevel 1 goto :failed

echo.
echo [6/6] Optimierte Release-Version bauen...
cargo build --release --locked
if errorlevel 1 goto :failed

copy /Y "target\release\media_backup_manager.exe" "Media Backup Manager.exe" >nul
if errorlevel 1 goto :failed

echo.
echo FERTIG:
echo %CD%\Media Backup Manager.exe
echo.
pause
exit /b 0

:failed
echo.
echo FEHLER: Das Projekt konnte nicht erstellt oder geprueft werden.
echo Bitte den kompletten Fehlertext kopieren oder fotografieren.
echo.
pause
exit /b 1
