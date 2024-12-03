open System
open System.Text.RegularExpressions
open Microsoft.FSharp.Core

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_3_input.txt")
let input = System.IO.File.ReadAllLines path

let findAllMuls (line: string)=
    Regex.Matches(line, @"mul\((\d+),(\d+)\)")
    |> Seq.map (fun m ->
        let p1 = m.Groups[1].ToString()
        let p2 = m.Groups[2].ToString()
        (Int32.Parse p1) * (Int32.Parse p2)
        )
    |> Seq.sum

let removeDontSections input = Regex.Replace(input, @"don't\(\)(.*?)(do\(\)|$)", "")

let part1 = input |> Seq.map findAllMuls |> Seq.sum
let part2 = String.Join("", input) |> _.ReplaceLineEndings("") |> removeDontSections |> findAllMuls