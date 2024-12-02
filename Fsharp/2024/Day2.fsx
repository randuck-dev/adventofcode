open System

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_2_input.txt")
let input = System.IO.File.ReadAllLines path


type Game = {
    data: int seq
    safe: bool
}
let solvePart1 (input: string array) =
    let data =
        input
        |> Seq.map (fun x -> x.Split(" ")
                             |> Seq.map Int32.Parse)
        |> Seq.map (fun x -> {data = x; safe = false})

    let safe =
        data
        |> Seq.filter (fun game ->
                game.data
                |> Seq.pairwise
                |> Seq.map (fun pair ->
                        match pair with
                        | a, b when Math.Abs (a-b) <= 3 && Math.Abs (a-b) >= 1 -> true
                        | _ -> false
                        )
                |> Seq.forall id
            )

    let allIncreasing =
        safe
        |> Seq.filter (fun x ->
            x.data
            |> Seq.pairwise |> Seq.forall (fun (a, b) -> a < b)
            )

    let allDecreasing =
        safe 
        |> Seq.filter (fun x ->
            x.data
            |> Seq.pairwise |> Seq.forall (fun (a, b) -> a > b)
            )

    let res = Seq.append allDecreasing allIncreasing
    printfn "%A" res
    Seq.length res


let part1 = solvePart1 input 

printfn $"Result: {part1}"
