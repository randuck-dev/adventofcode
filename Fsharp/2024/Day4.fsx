open System
open Microsoft.FSharp.Core

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_4_input.txt")
let input = System.IO.File.ReadAllLines path

let grid2d (data: string array) =
    data |> Array.map _.ToCharArray()

let validCoord(x, y, m, n) = 
    (x >= 0 && x < m && y >= 0 && y < n)

let rec findWord (index, word: string, grid: char array array, x: int, y, dirX: int, dirY: int) =
    if index = word.Length then true
    else if validCoord(x, y, grid.Length, grid[0].Length) && word[index] = grid[x][y] then
        findWord(index + 1, word, grid, x + dirX, y + dirY, dirX, dirY)
    else false
    
let searchWord (x: int list) (y: int list) (word: string) (grid: char array array) =
    let rows = grid.Length - 1
    let cols = grid[0].Length - 1
    
    let mutable ans = []
    
    let directions = x.Length - 1
    
    for row in [0..rows] do 
        for col in [0..cols] do
            let matches = [0..directions] |> List.where (fun v -> findWord(0, word, grid, row, col, x[v], y[v]))
            for m in matches do
                ans <- ans @ [[row; col]]
    
    ans
    
let searchWordWithChecks (grid: char array array) =
    let rows = grid.Length - 1
    let cols = grid[0].Length - 1
    
    let x = [-1; -1; -1; 0; 0; 1; 1; 1]
    let y = [-1; 0; 1; -1; 1; -1; 0; 1]
    
    let directions = x.Length - 1
    
    let mutable sub_grids: char array array array = [||] 
    for row in [0..rows] do 
        for col in [0..cols] do
            let areAllValid = List.forall (fun v ->
                let dirX = col + x[v]
                let dirY = row + y[v]
                validCoord(dirX,dirY,grid.Length,grid[0].Length)) [0..directions]
            if areAllValid then
                let top_left = grid[row - 1][col - 1]
                let top_middle = grid[row-1][col]
                let top_right = grid[row - 1][col + 1]
                let middle_left = grid[row][col-1]
                let center = grid[row][col]
                let middle_right = grid[row][col + 1]
                let lower_left = grid[row+1][col-1]
                let lower_middle = grid[row + 1][col]
                let lower_right = grid[row + 1][col+1]
                
                let sub_grid = [|
                    [|top_left; top_middle; top_right|]
                    [|middle_left;center; middle_right|]
                    [|lower_left;lower_middle;lower_right|]
                |]
                
                sub_grids <- Array.append sub_grids [|sub_grid|]

    sub_grids

let grid = input |> grid2d

let part_1_x = [-1; -1; -1; 0; 0; 1; 1; 1]
let part_1_y = [-1; 0; 1; -1; 1; -1; 0; 1]
let part1= grid |> searchWord part_1_x part_1_y "XMAS" |> List.map (fun x -> 1) |> List.sum

let part_2_x = [-1; -1; 1; 1]
let part_2_y = [-1; 1; -1; 1]
let part2 = grid
            |> searchWordWithChecks
            |> Array.map (searchWord part_2_x part_2_y "MAS")
            |> Array.where (fun x -> x.Length = 2)
            |> Array.map (fun x -> 1)
            |> Array.sum
            



