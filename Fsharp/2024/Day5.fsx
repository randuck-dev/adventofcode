open System
open Microsoft.FSharp.Core

let cwd = System.IO.Directory.GetCurrentDirectory()
let path = System.IO.Path.Combine(cwd, "2024/Inputs/day_5_test.txt")
let input = System.IO.File.ReadAllLines path

let whitespace_segment_length = 1
let page_rules = input |> Array.takeWhile (fun x -> not (x.Equals ""))
let pages = input |> Array.skip (page_rules.Length + whitespace_segment_length)

let getMatchingRules (v: string) (rules: string seq) =
    rules |> Seq.where (fun x -> x.StartsWith v)
    
let matchesAllRules curr (rest: string seq)=
    if Seq.isEmpty rest then true
    else 
        let matchingRules = getMatchingRules curr page_rules
        printfn "MatchingRules for current: Curr=%A Rules=%A" curr matchingRules
        matchingRules
        |> Seq.forall (fun rule ->
            let getSuccessor = rule.Split("|")[1]
            not (Seq.exists (fun x -> x.Equals(getSuccessor)) rest)
          )
let mutable goodPages: string array array = [||]
let mutable badPages: string array array = [||]

for page in pages do
    let split = page.Split ","
    let mutable allGood = true 
    for i = 0 to (split.Length - 1) do
        let curr = split[i]
        let before = split[0..i]
        let matches = matchesAllRules curr before
        if not(matches) then
            allGood <- false
    if allGood then
       goodPages <- Array.append goodPages [|split|]
    else
        badPages <- Array.append badPages [|split|]

let part1 = goodPages
            |> Seq.map (fun x -> Int32.Parse x[x.Length / 2])
            |> Seq.sum