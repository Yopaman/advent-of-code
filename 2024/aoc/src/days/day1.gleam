import gleam/int
import gleam/io
import gleam/list
import gleam/string
import helpers

pub fn parse() -> #(List(Int), List(Int)) {
  let assert Ok(input) = helpers.get_input("1")
  input
  |> string.split("\n")
  |> list.filter(fn(s) { s != "" })
  |> list.map(fn(e) {
    let assert Ok(value) = case string.split(e, "   ") {
      [first, second] -> {
        let assert Ok(f) = int.parse(first)
        let assert Ok(s) = int.parse(second)
        Ok(#(f, s))
      }
      _ -> Error(#(0, 0))
    }
    value
  })
  |> list.unzip
}

pub fn part1(input: #(List(Int), List(Int))) -> String {
  let #(first, second) = input
  let sorted_first = list.sort(first, int.compare)
  let sorted_second = list.sort(second, int.compare)
  list.map2(sorted_first, sorted_second, fn(f, s) { int.absolute_value(s - f) })
  |> int.sum
  |> int.to_string
}

pub fn part2(_input: #(List(Int), List(Int))) -> String {
  "todo"
}

pub fn main() {
  let input = parse()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input))
}
