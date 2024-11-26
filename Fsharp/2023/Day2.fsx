let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2023/Inputs/day_2_input.txt")
let input = System.IO.File.ReadAllLines path

type Game = {
    Id : int
    Blue : int
    Green : int
    Red : int
}

type Cube =
    | Red of int
    | Green of int
    | Blue of int

let sequence_into_cubes (input: string) =
    let s = input.Split(";")
    s
    |> Seq.map _.Trim()
    |> Seq.map (fun s ->
            s.Split(",")
            |> Seq.map (fun draw ->
                    let split = draw.Trim().Split(" ")
                    let count = split.[0]
                    let color = split.[1]
                    match color with
                    | "red" -> Red(int count)
                    | "green" -> Green(int count)
                    | "blue" -> Blue(int count)
                    | _ -> failwith "Unexpected color"
                )
        )
    
let into_game (input: string) =
    let id = int (input.Split(" ")[1])
    {
        Id = id
        Blue = 0
        Green = 0
        Red = 0
    }
     
let find_max (game: Game) (cube: Cube) =
    match cube with
    | Red r when r > game.Red -> { game with Red = r }
    | Green g when g > game.Green -> { game with Green = g}
    | Blue b when b > game.Blue -> { game with Blue = b }
    | _ -> game
    
let at_most_scoring game =
    if game.Red <= 12 && game.Green <= 13 && game.Blue <= 14 then game.Id else 0
    
let power_scoring game =
    game.Red * game.Green * game.Blue

let calculate_score input strategy scoring =
    let mutable data = 0
    for (game, cubes) in input do
        let game = cubes |> Seq.collect id
                        |> Seq.fold strategy game
        data <- data + scoring game
    data
    
let parsed_data = input
                |> Seq.map (fun n -> n.Split(":"))
                |> Seq.map (fun n -> (into_game n.[0], sequence_into_cubes n.[1]))
                |> Seq.toList
        
let part1 = calculate_score parsed_data find_max at_most_scoring
let part2 = calculate_score parsed_data find_max power_scoring
    
printfn $"Result: {part1}"
printfn $"Result: {part2}"
