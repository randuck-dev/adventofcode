module Year2023Day1

open System
open Library.Utils

let rec reduceString (str, numbers) =
    match str with
    | ("" | null) -> numbers
    | x ->
        let firstChar = x[0]

        if (int) firstChar >= 48 && (int) firstChar <= 57 then
            reduceString (x[1..], numbers @ [ firstChar ])
        else
            reduceString (x[1..], numbers)

let part1 (data) =
    data
    |> Seq.map (fun n ->
        let chars = reduceString (n, [])
        let start = chars[0]
        let last = Seq.last chars
        let combined = String.Concat(start, last)

        let numb = Int32.Parse combined
        numb)
    |> Seq.sum


let solver =
    { parse = (fun n -> readAllLines(n, asString))
      part1 = part1
      part2 = part1 }
