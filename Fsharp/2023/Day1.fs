module Year2023Day1

open System
open Common.Utils

let rec reduceString (str, numbers) =
    match str with
    | ("" | null) -> List.rev numbers
    | x ->
        match str[0] with
        | d when Char.IsDigit d -> reduceString (x[1..], str[0] :: numbers)
        | _ -> reduceString (x[1..], numbers)

let part1 data =
    data
    |> Seq.choose (fun n ->
        match reduceString(n, []) with
        | [] -> None
        | data ->
            let first = data |> Seq.head
            let last = data |> Seq.last
            let combined = sprintf "%c%c" first last
            combined |> int |> Some
    )
    |> Seq.sum

let rec reduceString2 (str, numbers) =
    match str with
    | ("" | null) -> numbers
    | x ->
        let firstChar = x[0]
        let value = match x with
                    | n when n.StartsWith "one" -> Some ('1', 3)
                    | n when n.StartsWith "two" -> Some ('2', 3)
                    | n when n.StartsWith "three" -> Some ('3', 5)
                    | n when n.StartsWith "four" -> Some ('4', 4)
                    | n when n.StartsWith "five" -> Some ('5', 4)
                    | n when n.StartsWith "six" -> Some ('6', 3)
                    | n when n.StartsWith "seven" -> Some ('7', 5)
                    | n when n.StartsWith "eight" -> Some ('8', 5)
                    | n when n.StartsWith "nine" -> Some ('9', 4)
                    | _ -> None
        
        match value with
        | Some (v, substring) -> reduceString2 (x[substring..], numbers @ [v])
        | None -> 
            if (int) firstChar >= 48 && (int) firstChar <= 57 then
                reduceString2 (x[1..], numbers @ [ firstChar ])
            else
                reduceString2 (x[1..], numbers)
let part2 data =
    data
    |> Seq.map (fun n ->
        let chars = reduceString2(n, [])
        let start = chars[0]
        let last = Seq.last chars
        let combined = String.Concat(start, last)

        let numb = Int32.Parse combined
        numb
        )
    |> Seq.sum


let solver =
    { parse = (fun n -> readAllLines(n, id))
      part1 = part1
      part2 = part2 }
