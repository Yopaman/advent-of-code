import gleam/io
import helpers

pub fn parse() -> String {
  let assert Ok(input) = helpers.get_input("")
  input
}

pub fn part1(_input: String) -> String {
  "todo"
}

pub fn part2(_input: String) -> String {
  "todo"
}

pub fn main() {
  let input = parse()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input))
}
