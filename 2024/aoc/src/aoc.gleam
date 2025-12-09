import argv
import days/day1
import days/day2
import days/day3
import days/day4
import days/day5
import days/day6
import days/day7
import dotenv_gleam
import envoy
import gleam/hackney
import gleam/http/request
import gleam/io
import gleam/result
import simplifile

pub fn main() {
  dotenv_gleam.config()

  case argv.load().arguments {
    ["get", day] -> {
      let assert Ok(input) =
        day
        |> get_input
      let assert Ok(_) = simplifile.write("inputs/day" <> day <> ".txt", input)
      Nil
    }
    [day] -> {
      case day {
        "1" -> {
          day1.main()
        }
        "2" -> {
          day2.main()
        }
        "3" -> {
          day3.main()
        }
        "4" -> {
          day4.main()
        }
        "5" -> {
          day5.main()
        }
        "6" -> {
          day6.main()
        }
        "7" -> {
          day7.main()
        }
        _ -> io.println("Unknown day!")
      }
    }
    _ -> io.println("An error happened!")
  }
}

pub fn get_input(day: String) -> Result(String, Nil) {
  use req <- result.try(request.to(
    "https://adventofcode.com/2024/day/" <> day <> "/input",
  ))
  use res <- result.try(
    req
    |> request.prepend_header(
      "Cookie",
      "session=" <> result.unwrap(envoy.get("AOC_SESSION"), "null"),
    )
    |> hackney.send
    |> result.replace_error(Nil),
  )
  Ok(res.body)
}
