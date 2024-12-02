import gleam/int
import gleam/io
import gleam/list
import gleam/string
import helpers

pub fn parse() -> List(List(Int)) {
  let assert Ok(input) = helpers.get_input("2")
  let lines =
    input
    |> string.split("\n")
  lines
  |> list.map(fn(l) {
    l
    |> string.split(" ")
    |> list.map(fn(c) {
      let assert Ok(n) = int.parse(c)
      n
    })
  })
}

pub fn is_report_safe(report: List(Int)) -> Bool {
  let win_list = report |> list.window_by_2
  list.all(win_list, fn(e) {
    let #(a, b) = e
    int.absolute_value(a - b) <= 3
  })
  && {
    list.all(win_list, fn(e) {
      let #(a, b) = e
      a < b
    })
    || list.all(win_list, fn(e) {
      let #(a, b) = e
      a > b
    })
  }
}

pub fn part1(input: List(List(Int))) -> String {
  input
  |> list.filter(is_report_safe)
  |> list.length
  |> int.to_string
}

pub fn bruteforce(report: List(Int)) -> Bool {
  bruteforce_loop([], report)
}

fn bruteforce_loop(first_part: List(Int), last_part: List(Int)) -> Bool {
  case last_part {
    [first_elem, ..rest] -> {
      case is_report_safe(first_part |> list.append(rest)) {
        True -> True
        False -> bruteforce_loop(first_part |> list.append([first_elem]), rest)
      }
    }
    [] -> False
  }
}

pub fn part2(input: List(List(Int))) -> String {
  input
  |> list.filter(bruteforce)
  |> list.length
  |> int.to_string
}

pub fn main() {
  let input = parse()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input))
}
