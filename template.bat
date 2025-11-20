@echo off

echo. >> src\solution_template.rs
echo pub fn solution(input: String) -^> String { >> src\solution_template.rs
echo. >> src\solution_template.rs
echo     format!("{:?}",input) >> src\solution_template.rs
echo } >> src\solution_template.rs


setlocal enabledelayedexpansion

for /l %%i in (1, 1, 12) do (
    set DayNumber=%%i

    if not exist data\day!DayNumber!.txt type NUL > data\day!DayNumber!.txt
    if not exist data\day!DayNumber!test.txt type NUL > data\day!DayNumber!test.txt

    if not exist src\day!DayNumber!part1.rs >> .gitignore echo.
    if not exist src\day!DayNumber!part1.rs >> .gitignore <nul set /p "=day!DayNumber!*"
    
    if not exist src\day!DayNumber!part1.rs copy src\solution_template.rs src\day!DayNumber!part1.rs
    if not exist src\day!DayNumber!part2.rs copy src\solution_template.rs src\day!DayNumber!part2.rs
    
    
)

del src\solution_template.rs