open Microsoft.FSharp.Core

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_4_test.txt")
let input = System.IO.File.ReadAllLines path

let grid2d (data: string array) =
    data |> Array.map _.ToCharArray()

let validCoord(x, y, m, n) = 
    (x >= 0 && x < m && y >= 0 && y < n)

// This function searches for the given word
// in a given direction from the coordinate.
let rec findWord (index, word: string, grid: char array array, x: int, y, dirX: int, dirY: int) =
    try
        if index = word.Length then true
        else if validCoord(x, y, grid.Length, grid[0].Length) && word[index] = grid[x][y] then
            findWord(index + 1, word, grid, x + dirX, y + dirY, dirX, dirY)
        else false
    with
    | :? System.Exception as ex ->
        printfn "%A" ex.Message
        false
    
let searchWord (word: string) (grid: char array array) =
    let rows = grid.Length - 1
    let cols = grid[0].Length - 1
    
    let mutable ans = []
    let x = [-1; -1; -1; 0; 0; 1; 1; 1]
    let y = [-1; 0; 1; -1; 1; -1; 0; 1]
    
    let directions = x.Length - 1
    
    for row in [0..rows] do 
        for col in [0..cols] do
            let matches = [0..directions] |> List.where (fun v -> findWord(0, word, grid, row, col, x[v], y[v]))
            for m in matches do
                ans <- ans @ [[row; col]]
    
    ans
    
let block row col = (row / 3) * 3 + (col / 3)

let mutable mapping = System.Collections.Concurrent.ConcurrentDictionary() 
let searchWordWithChecks (word: string) (grid: char array array) =
    let rows = grid.Length - 1
    let cols = grid[0].Length - 1
    
    let x = [-1; -1; -1; 0; 0; 1; 1; 1]
    let y = [-1; 0; 1; -1; 1; -1; 0; 1]
    
    let directions = x.Length - 1
    
    for row in [0..rows] do 
        for col in [0..cols] do
            let matches = [0..directions] |> List.where (fun v -> findWord(0, word, grid, row, col, x[v], y[v]))
            for m in matches do
                let b = block row col
                mapping.AddOrUpdate(b, 1, (fun a b -> b + 1)) |> ignore

let grid = input |> grid2d
let part1= grid |> searchWord "XMAS"

let part2 = grid |> searchWordWithChecks "MAS"

mapping

