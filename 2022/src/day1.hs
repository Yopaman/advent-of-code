import Data.List.Split as Split
import Data.List

main = do
  content <- readFile "../inputs/day1.txt"
  let input = lines content
  let result1 = maximum $ calories input
  let result2 = sum . take 3 . reverse . sort $ calories input
  putStrLn $ "part 1 : " ++ show result1
  putStrLn $ "part 2 : " ++ show result2

calories input = map (sum . map read) $ Split.splitOn [""] input

