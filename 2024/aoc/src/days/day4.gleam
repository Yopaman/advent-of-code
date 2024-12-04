import gleam/int
import gleam/io
import gleam/list
import gleam/regex
import gleam/string
import helpers

pub fn parse() -> List(String) {
  let assert Ok(input) = helpers.get_input("4")
  input
  |> string.split("\n")
}

pub fn count_horizontal(input: List(String), word: String) -> Int {
  let assert Ok(re) =
    regex.from_string("(?=(" <> word <> "|" <> string.reverse(word) <> "))")
  input
  |> list.map(fn(l) {
    re
    |> regex.scan(l)
    |> list.length
  })
  |> int.sum
}

pub fn count_diagonal_rl(input: List(String), word: String) -> Int {
  let length = input |> list.length
  input
  |> list.index_map(fn(l, i) {
    let s_length = string.length(l)
    l
    |> string.pad_start(s_length + i, ".")
    |> string.pad_end(s_length + length - i, ".")
  })
  |> count_vertical(word)
}

pub fn count_diagonal_lr(input: List(String), word: String) -> Int {
  let length = input |> list.length
  input
  |> list.index_map(fn(l, i) {
    let s_length = string.length(l)
    l
    |> string.pad_start(s_length + length - i, ".")
    |> string.pad_end(s_length + i, ".")
  })
  |> count_vertical(word)
}

pub fn count_vertical(input: List(String), word: String) -> Int {
  let chars = input |> list.map(fn(s) { s |> string.to_graphemes })

  chars
  |> list.transpose
  |> list.map(fn(s) { s |> string.join("") })
  |> count_horizontal(word)
}

pub fn part1(input: List(String)) -> String {
  let h = input |> count_horizontal("XMAS")
  let v = input |> count_vertical("XMAS")
  let rl = input |> count_diagonal_rl("XMAS")
  let lr = input |> count_diagonal_lr("XMAS")
  h + v + rl + lr |> int.to_string
}

// [[[a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a]],
//   [[a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a]],
//   [[a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a], [a,a,a]]]

pub fn part2(input: List(String)) -> String {
  input
  |> list.map(fn(l) { string.to_graphemes(l) })
  |> list.map(fn(l) { list.window(l, 3) })
  |> list.window(3)
  |> list.map(fn(big_list) {
    let assert Ok(a_l) = list.first(big_list)
    let assert Ok(b_l) = big_list |> list.drop(1) |> list.first
    let assert Ok(c_l) = list.last(big_list)
    big_list_loop(a_l, b_l, c_l, 0)
  })
  |> int.sum
  |> int.to_string
}

fn big_list_loop(
  a_l: List(List(String)),
  b_l: List(List(String)),
  c_l: List(List(String)),
  counter: Int,
) -> Int {
  case a_l, b_l, c_l {
    [["M", _, "M"], ..tail_a],
      [[_, "A", _], ..tail_b],
      [["S", _, "S"], ..tail_c]
    | [["M", _, "S"], ..tail_a],
      [[_, "A", _], ..tail_b],
      [["M", _, "S"], ..tail_c]
    | [["S", _, "M"], ..tail_a],
      [[_, "A", _], ..tail_b],
      [["S", _, "M"], ..tail_c]
    | [["S", _, "S"], ..tail_a],
      [[_, "A", _], ..tail_b],
      [["M", _, "M"], ..tail_c]
    -> big_list_loop(tail_a, tail_b, tail_c, counter + 1)
    [_, ..tail_a], [_, ..tail_b], [_, ..tail_c] ->
      big_list_loop(tail_a, tail_b, tail_c, counter)
    [], [], [] -> counter
    _, _, _ -> panic as "error"
  }
}

pub fn main() {
  let input = parse()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input))
}
