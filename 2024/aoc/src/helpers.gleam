import simplifile

pub fn get_input(day: String) -> Result(String, simplifile.FileError) {
  simplifile.read("inputs/day" <> day <> ".txt")
}
