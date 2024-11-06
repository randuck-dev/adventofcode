module Library.Utils

open System
open System.Net
open System.Net.Http

type Day<'a, 'b, 'c> = {parse: string -> 'a; part1: 'a -> 'b; part2: 'a -> 'c}

let fetchInput(year, day) =
    let envSession = Environment.GetEnvironmentVariable "AOC_SESSION_COOKIE"
    async {
        let baseAddress = Uri("https://adventofcode.com")
        let path = $"/{year}/day/{day}/input"
        let cookieContainer = CookieContainer()
        cookieContainer.Add(baseAddress, Cookie("session", envSession))
        
        use handler = new HttpClientHandler(CookieContainer = cookieContainer)
        use httpClient = new HttpClient(handler)
        httpClient.BaseAddress <- baseAddress

        let! response = httpClient.GetStringAsync(path) |> Async.AwaitTask
        return response
    }


let asString : string -> string = id
let readAllLines (data: string, f) =
    let w = data |> _.Split(Environment.NewLine) |> Seq.where (fun n -> not (String.IsNullOrEmpty(n)))
    
    w |> Seq.map (fun n -> f(n))
    