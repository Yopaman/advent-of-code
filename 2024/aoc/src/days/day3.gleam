import gleam/int
import gleam/io
import gleam/list
import gleam/option
import gleam/regex
import gleam/result
import gleam/string
import helpers

pub fn parse() -> List(#(Int, Int)) {
  let assert Ok(input) = helpers.get_input("3")
  let assert Ok(re) = regex.from_string("mul\\(([0-9]+),([0-9]+)\\)")
  re
  |> regex.scan(input)
  |> list.map(fn(m) {
    let assert option.Some(n1_s) =
      list.first(m.submatches) |> result.unwrap(option.Some("0"))
    let assert option.Some(n2_s) =
      list.last(m.submatches) |> result.unwrap(option.Some("0"))
    let assert Ok(n1) = int.parse(n1_s)
    let assert Ok(n2) = int.parse(n2_s)
    #(n1, n2)
  })
}

pub fn part1(input: List(#(Int, Int))) -> String {
  input
  |> list.map(fn(op) {
    let #(n1, n2) = op
    n1 * n2
  })
  |> int.sum
  |> int.to_string
}

pub type Instruction {
  Do
  Dont
  Mul(Int, Int)
}

pub fn parse2() -> List(Instruction) {
  let assert Ok(input) = helpers.get_input("3")
  let assert Ok(re) =
    regex.from_string("(mul\\(([0-9]+),([0-9]+)\\))|(do\\(\\))|(don't\\(\\))")
  re
  |> regex.scan(input)
  |> list.map(fn(m) {
    io.debug(m.content)
    case m.content {
      "do()" -> Do
      "don't()" -> Dont
      "mul(" <> numbers -> {
        let parsed_string = string.drop_end(numbers, 1)
        let ns = string.split(parsed_string, ",")
        let assert Ok(n1_s) = list.first(ns)
        let assert Ok(n2_s) = list.last(ns)
        let assert Ok(n1) = int.parse(n1_s)
        let assert Ok(n2) = int.parse(n2_s)
        Mul(n1, n2)
      }
      _ -> panic as "???"
    }
  })
}

pub fn part2(input: List(Instruction)) -> String {
  part2_loop(input, True, 0)
}

pub fn part2_loop(
  input: List(Instruction),
  enabled: Bool,
  result: Int,
) -> String {
  case input {
    [Do, ..tail] -> part2_loop(tail, True, result)
    [Dont, ..tail] -> part2_loop(tail, False, result)
    [Mul(n1, n2), ..tail] if enabled -> part2_loop(tail, True, result + n1 * n2)
    [Mul(_, _), ..tail] if !enabled -> part2_loop(tail, False, result)
    [] -> result |> int.to_string
    [_] -> panic as "not possible ??"
    [Mul(_, _), _] -> panic as "not possible ??"
    [_, _, _, ..] -> panic as "not possible ??"
  }
}

pub fn main() {
  let input = parse()
  let input2 = parse2()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input2))
}
