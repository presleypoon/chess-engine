@echo off
set RUST_BACKTRACE=1
cargo run
copy target\debug\chess.exe .
pause
exit
