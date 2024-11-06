open Library.Utils
    
let run day =
    let runner day solver =
        let rawData = fetchInput(2023, day) |> Async.RunSynchronously
        let data = solver.parse rawData 
        let p1Result = solver.part1 data
        let p2Result = solver.part2 data
        
        printfn $"Day({day}): P1={p1Result} P2={p2Result}"
    
    match day with
    | 1 -> runner 1 Year2023Day1.solver
    | _ -> failwith "Unexpected day"
    
    
run 1