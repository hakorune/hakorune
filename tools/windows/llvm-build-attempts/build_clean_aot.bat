@echo off
chcp 437 >nul
set "LLVM_SYS_180_PREFIX=C:\LLVM-18"
set "LLVM_SYS_180_FFI_WORKAROUND=1" 
set "LLVM_SYS_180_NO_LIBFFI=1"
set "PATH=C:\LLVM-18\bin;%PATH%"

echo Cleaning and building Hakorune with LLVM AOT (no libffi)...
cargo clean
cargo build --bin hakorune --release --features llvm

echo.
echo Checking output...
if exist target\release\hakorune.exe (
    echo SUCCESS: hakorune.exe created
    dir target\release\hakorune.exe
) else (
    echo ERROR: hakorune.exe not found
)
