open System

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_1_input.txt")
let input = System.IO.File.ReadAllLines path

let parse (x: string)=
    let data = x.Split "   "
    [Int32.Parse data[0]; Int32.Parse data[1]]

let mapped = Seq.map parse input

let solvePart1 input =
    let leftList = Seq.map (fun (x: int list) -> x.[0]) input |> Seq.sort |> Seq.toList
    let rightList = Seq.map (fun (x: int list) -> x.[1]) input |> Seq.sort |> Seq.toList

    (0, leftList, rightList)
    |||> Seq.fold2 (
                    fun acc left right ->
                        acc + Math.Abs (right - left)
                    )
    

let solvePart2 input =
    let leftList = Seq.map (fun (x: int list) -> x.[0]) input
    let rightList = Seq.map (fun (x: int list) -> x.[1]) input

    let rightGroupings = Seq.groupBy (fun x -> x) rightList
                                |> Seq.map (fun (value, occurrences) -> (value, Seq.length occurrences) )
                                |> Map.ofSeq

    Seq.map (fun x ->
            match rightGroupings.TryGetValue x with
            | true, value -> x * value
            | _ -> 0 
            ) leftList
            |> Seq.sum


let part1 = solvePart1 mapped
let part2 = solvePart2 mapped

printfn $"Result: {part1}"
printfn $"Result: {part2}"
