
main = do
  content <- readFile "../inputs/day4.txt"
  let input = lines content
  let parsedLines = map readLine input
  let res1 = length $ filter part1 parsedLines
  let res2 = length $ filter part2 parsedLines
  putStrLn $ "part 1 = " ++ show res1
  putStrLn $ "part 2 = " ++ show res2
numbers :: String -> (Int, Int)
numbers range = 
  let (a, a') = break (== '-') range in
  (read a, read $ tail a')

readLine :: String -> ((Int, Int), (Int, Int))
readLine line = 
  let (range1, range2) = break (== ',') line in
  (numbers range1, numbers $ tail range2)

part1 :: ((Int, Int), (Int, Int)) -> Bool
part1 ((a, a'), (b, b')) = (a <= b && a' >= b') || (a >= b && a' <= b')

part2 :: ((Int, Int), (Int, Int)) -> Bool
part2 ((a, a'), (b, b')) = not $ (b < a && b' < a) || (a < b && a' < b)


