import Data.List
import Data.List.Split
import qualified Data.Matrix as M

main = do
  content <- readFile "../inputs/day5.txt"
  let input = lines content
  let (infos, insts) = (init $ takeWhile (/= "") input, tail $ dropWhile (/= "") input)
  putStrLn $ show $ "part 1 = " ++ show (part1 infos insts)
  putStrLn $ show $ "part 2 = " ++ show (part2 infos insts)

getCrates :: String -> [String]
getCrates line = map (tail . take 2) $ chunksOf 4 line

makeCratesLists :: [[String]] -> [[String]]
makeCratesLists lines = map (filter (/= " ")) (M.toLists . M.transpose . M.fromLists $ lines) 

parseCrates :: [String] -> [[String]]
parseCrates infos = makeCratesLists $ map getCrates infos

parseInst :: String -> (Int, Int, Int)
parseInst inst = 
  let w = words inst in 
  (read $ w!!1, read $ w!!3, read $ w!!5)

executeInstV1 :: String -> [[String]] -> [[String]]
executeInstV1 insts crates = 
  zipWith (\i x ->
    if i == to
    then (reverse $ take amount (crates!!(from - 1))) ++ x
    else 
      if i == from
      then drop amount x 
      else x) 
  [1..] crates
  where (amount, from, to) = parseInst insts

execAllInstsV1 :: [[String]] -> [String] -> [[String]]
execAllInstsV1 input insts = foldl (\acc x -> executeInstV1 x acc) input insts

executeInstV2 :: String -> [[String]] -> [[String]]
executeInstV2 insts crates = 
  zipWith (\i x ->
    if i == to
    then (take amount (crates!!(from - 1))) ++ x
    else 
      if i == from
      then drop amount x 
      else x) 
  [1..] crates
  where (amount, from, to) = parseInst insts

execAllInstsV2 :: [[String]] -> [String] -> [[String]]
execAllInstsV2 input insts = foldl (\acc x -> executeInstV2 x acc) input insts

part1 :: [String] -> [String] -> [String]
part1 infos insts = map head (execAllInstsV1 (parseCrates infos) insts)

part2 :: [String] -> [String] -> [String]
part2 infos insts = map head (execAllInstsV2 (parseCrates infos) insts)
