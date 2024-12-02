open System

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_2_input.txt")
let input = System.IO.File.ReadAllLines path

let shrink (report: int seq) =
    seq {
        yield report
        let length = (report |> Seq.length) - 1
        for i in [0..length] -> report |> Seq.removeAt i
    }

let toData (input: string array)=
    input
    |> Seq.map (fun x -> x.Split(" ") |> Seq.map Int32.Parse)

let isWithinBounds report =
    let withinBounds (x: int) (y: int) =
        let diff = abs (x - y)
        1 <= diff && diff <= 3

    report
    |> Seq.pairwise
    |> Seq.forall (fun (a, b) -> (withinBounds a b))

// moving across the data to figure out if every pair is monotonically increasing
let allIncreasing input =
    input |> Seq.pairwise |> Seq.forall (fun (a, b) -> a < b)

// moving across the data to figure out if every is monotonically decreasing
let allDecreasing input =
        input |> Seq.pairwise |> Seq.forall (fun (a, b) -> a > b)

let isSafe report =
    (isWithinBounds report) && (allIncreasing report || allDecreasing report)

let isSafeBruteForce report =
    let reports = shrink report
    reports |> Seq.exists isSafe

let solvePart1 (input: string array) =
    let safe = toData input |> Seq.filter isSafe
    Seq.length safe

let solvePart2 (input: string array) =
    let safe = toData input |> Seq.filter isSafeBruteForce
    Seq.length safe

let part1 = solvePart1 input
let part2 = solvePart2 input

printfn $"Result: {part1}"
printfn $"Result: {part2}"
