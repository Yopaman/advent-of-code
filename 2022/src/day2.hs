main = do
  content <- readFile "../inputs/day2.txt"
  let input = lines content
  let part1 = sum $ map getScore input
  let part2 = sum $ map getScore2 input
  putStrLn $ "Part 1 = " ++ show part1
  putStrLn $ "Part 2 = " ++ show part2

getScore :: String -> Int
getScore game
  | (me, opp) == ('X', 'C') 
    || (me, opp) == ('Y', 'A') 
    || (me, opp) == ('Z', 'B') = 6 + shapeScore me
  | (me, opp) == ('X', 'A')
    || (me, opp) == ('Y', 'B')
    || (me, opp) == ('Z', 'C') = 3 + shapeScore me
  | otherwise = shapeScore me
  where (me, opp) = (last game, head game)

getScore2 game = case need of
  'X' -> shapeScore $ getLosingShape opp
  'Y' -> 3 + (shapeScore $ getDrawShape opp)
  'Z' -> 6 + (shapeScore $ getWinningShape opp)
  where (need, opp) = (last game, head game)

shapeScore :: Char -> Int
shapeScore shape = case shape of 
  'X' -> 1
  'Y' -> 2
  'Z' -> 3
  _ -> 0

getWinningShape shape = case shape of
  'A' -> 'Y'
  'B' -> 'Z'
  'C' -> 'X'

getDrawShape shape = case shape of
  'A' -> 'X'
  'B' -> 'Y'
  'C' -> 'Z'

getLosingShape shape = case shape of
  'A' -> 'Z'
  'B' -> 'X'
  'C' -> 'Y'


