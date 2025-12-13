@echo off

setlocal enabledelayedexpansion

for /l %%i in (1, 1, 12) do (
    set DayNumber=%%i

    if not exist data\day!DayNumber!.txt type NUL > data\day!DayNumber!.txt

    if not exist src\day!DayNumber!part1.rs >> .gitignore echo.
    if not exist src\day!DayNumber!part1.rs >> .gitignore <nul set /p "=day!DayNumber!*"
    
    if not exist src\day!DayNumber!part1.rs copy template_skeleton.rs src\day!DayNumber!part1.rs
    if not exist src\day!DayNumber!part2.rs copy template_skeleton.rs src\day!DayNumber!part2.rs
    
    
)
del src\day12part2.rs