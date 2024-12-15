open System
open Microsoft.FSharp.Core

let grid2d (data: string array) =
    let mutable grid = Array.create data.Length [||]
    for i in [0..data.Length - 1] do
        let row = data[i]
        let rowData = row.ToCharArray()
        grid.[i] <- rowData
        
    grid
    
let validCoord(x, y, m, n) = 
    (x >= 0 && x < m && y >= 0 && y < n)

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_6_input.txt")
let input = System.IO.File.ReadAllLines path

type Position = {
    x: int
    y: int
}

type Movement =
    | UP
    | DOWN
    | LEFT
    | RIGHT
    | STOP

type GuardPosition = {
    x: int
    y: int
    unique_positions: Set<Position>
    current_trajectory: Movement
    
}

let possiblePositions = [|'^';'v'; '>'; '<'|] |> Set.ofArray

let isGuard c = possiblePositions.Contains c
    
let findGuard (grid: char array array)=
    let mutable guard = None
    for i in [0..grid.Length - 1] do
        let row = grid.[i]
        for j in [0..row.Length - 1] do
            if isGuard row.[j] then
                let currentTrajectory = match row.[j] with
                                        | '^' -> UP
                                        | 'v' -> DOWN
                                        | '<' -> LEFT
                                        | '>' -> RIGHT
                                        | _ -> failwith "Did"
                guard <- Some {x=j; y=i; unique_positions = Set.empty; current_trajectory = currentTrajectory}
    guard.Value

let outOfMap (guard: GuardPosition) (grid: char array array) =
    not (validCoord(guard.x, guard.y,grid.Length, grid[0].Length))

type PositionMove =
    | Move
    | Rotate
    | OutOfMap
    
let DetermineNextMove x y (grid: char array array) =
    try
        match grid.[y][x] with
        | '#' -> Rotate 
        | '.' -> Move 
        | '^' -> Move 
        | _ -> OutOfMap
    with
    | :? System.IndexOutOfRangeException -> OutOfMap
    

let rec getNextAllowedMovement (guard: GuardPosition) (grid: char array array) =
    let current_direction = guard.current_trajectory
    let current_position = {x = guard.x; y = guard.y}
    let updatedSet = guard.unique_positions.Add current_position

    match current_direction with
    | UP -> match DetermineNextMove guard.x (guard.y - 1) grid with
            | Move -> {guard with current_trajectory = UP; y = guard.y - 1; unique_positions = updatedSet}
            | Rotate -> getNextAllowedMovement {guard with current_trajectory = RIGHT} grid
            | OutOfMap -> {guard with unique_positions = updatedSet; current_trajectory = STOP}
    | DOWN -> match DetermineNextMove guard.x (guard.y + 1) grid with
                | Move -> {guard with current_trajectory = DOWN; y = guard.y + 1; unique_positions = updatedSet}
                | Rotate -> getNextAllowedMovement {guard with current_trajectory = LEFT} grid
                | OutOfMap -> {guard with unique_positions = updatedSet; current_trajectory = STOP}
    | LEFT -> match DetermineNextMove (guard.x - 1) guard.y grid with
                | Move -> {guard with current_trajectory = LEFT; x = guard.x - 1; unique_positions = updatedSet}
                | Rotate -> getNextAllowedMovement {guard with current_trajectory = UP} grid
                | OutOfMap -> {guard with unique_positions = updatedSet; current_trajectory = STOP}
    | RIGHT -> match DetermineNextMove (guard.x + 1) guard.y grid with
                | Move -> {guard with current_trajectory = RIGHT ; x = guard.x + 1; unique_positions = updatedSet}
                | Rotate -> getNextAllowedMovement {guard with current_trajectory = DOWN} grid
                | OutOfMap -> {guard with unique_positions = updatedSet; current_trajectory = STOP}
    | STOP -> guard

let rec moveOutOfMap grid guard =
    if guard.current_trajectory = STOP then
        guard
    else
        getNextAllowedMovement guard grid
        |> moveOutOfMap grid


let predictExit grid =
    // given the guard we can now make it move around the map and predict where they would be exiting
    // let's do this recursively
    grid |> findGuard |> moveOutOfMap grid

let uniquePositions guard =
    guard.unique_positions.Count

let part1 = input
            |> grid2d
            |> predictExit
            |> uniquePositions

assert (part1 = 41)
printfn "%A" part1