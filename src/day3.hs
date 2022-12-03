import Data.List (intersect, nub)
import Data.List.Split (chunksOf)
import Data.Char (ord)

main = do
  content <- readFile "../inputs/day3.txt"
  let input = lines content
  let part1 = sum $ map priorities input
  let part2 = sum $ map (priority . commonLetter) (chunksOf 3 input)
  putStrLn $ "part 1 = " ++ show part1
  putStrLn $ "part 2 = " ++ show part2

splitInput :: String -> (String, String)
splitInput input = splitAt (length input `div` 2) input

priority :: Char -> Int
priority item = 
  let toI = ord item in
    if toI >= 97 then toI - 96
    else toI - 38

priorities :: String -> Int
priorities rucksack =
  let (fst, snd) = splitInput rucksack in
    sum $ map priority $ nub $ intersect fst snd

commonLetter :: [String] -> Char
commonLetter group = head $ nub $ foldl intersect (head group) (tail group)
  
