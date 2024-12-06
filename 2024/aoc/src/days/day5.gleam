import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/order.{type Order}
import gleam/result
import gleam/string
import helpers

pub type PagesOrdering =
  Dict(String, List(String))

pub type PagesUpdates =
  List(List(String))

pub fn parse() -> #(PagesOrdering, PagesUpdates) {
  let assert Ok(input) = helpers.get_input("5")
  let assert Ok(#(ordering_s, updates_s)) =
    input |> string.trim |> string.split_once("\n\n")
  let pages_ordering =
    ordering_s
    |> string.split("\n")
    |> list.fold(dict.new(), fn(acc, s) {
      let assert Ok(#(x, y)) = string.split_once(s, "|")
      case dict.get(acc, x) {
        Ok(v) -> {
          let l = [y, ..v]
          dict.insert(acc, x, l)
        }
        Error(_) -> dict.insert(acc, x, [y])
      }
    })
  let pages_updates =
    updates_s |> string.split("\n") |> list.map(fn(s) { string.split(s, ",") })
  #(pages_ordering, pages_updates)
}

pub fn compare_pages(ordering: PagesOrdering, a: String, b: String) -> Order {
  let la = dict.get(ordering, a)
  let lb = dict.get(ordering, b)
  case la, lb {
    Ok(_), Error(_) -> order.Lt
    Error(_), Ok(_) -> order.Gt
    Ok(la), Ok(lb) -> {
      case list.contains(la, b), list.contains(lb, a) {
        True, False -> order.Lt
        False, True -> order.Gt
        _, _ -> order.Eq
      }
    }
    _, _ -> order.Eq
  }
}

pub fn middle_number(l: List(String)) -> Int {
  let length = list.length(l)
  let assert Ok(res) =
    l
    |> list.take(length / 2 + 1)
    |> list.last()
    |> result.unwrap("0")
    |> int.parse()
  res
}

pub fn part1(input: #(PagesOrdering, PagesUpdates)) -> String {
  let #(pages_ordering, pages_updates) = input
  pages_updates
  |> list.filter(fn(l) {
    case list.sort(l, fn(a, b) { compare_pages(pages_ordering, a, b) }) {
      l2 if l2 == l -> True
      _ -> False
    }
  })
  |> list.map(middle_number)
  |> int.sum
  |> int.to_string
}

pub fn part2(input: #(PagesOrdering, PagesUpdates)) -> String {
  let #(pages_ordering, pages_updates) = input
  pages_updates
  |> list.filter_map(fn(l) {
    case list.sort(l, fn(a, b) { compare_pages(pages_ordering, a, b) }) {
      l2 if l2 != l -> Ok(l2)
      _ -> Error(0)
    }
  })
  |> list.map(middle_number)
  |> int.sum
  |> int.to_string
}

pub fn main() {
  let input = parse()
  io.println("Part 1 : " <> part1(input))
  io.println("Part 2 : " <> part2(input))
}
