@echo off
chcp 437 >nul
set "LLVM_SYS_180_PREFIX=C:\LLVM-18"
set "LLVM_SYS_180_FFI_WORKAROUND=1"
set "PATH=C:\LLVM-18\bin;%PATH%"

echo Building Hakorune executable with LLVM support without libffi...
set "LLVM_SYS_NO_LIBFFI=1"
cargo build --bin hakorune --release --features llvm

echo.
echo Checking output...
if exist target\release\hakorune.exe (
    echo SUCCESS: hakorune.exe created
    dir target\release\hakorune.exe
) else (
    echo ERROR: hakorune.exe not found
    echo Listing exe files:
    dir target\release\*.exe
)
