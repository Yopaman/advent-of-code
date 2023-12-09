import Data.List
import qualified Data.List.Split as Split

main = do
  content <- readFile "../inputs/day6.txt"
  -- putStrLn $ show $ (zip [1..] (Split.divvy 4 1 content))
  putStrLn $ show $ startOfPacket content
  putStrLn $ show $ startOfMessage content

containsDistincts :: String -> Bool
containsDistincts element = (length . group . sort) element == length element

startOfPacket :: String -> Int
startOfPacket input = foldr (\(i, x) acc -> if containsDistincts x then (i + 3) else acc) 0 (zip [1..] (Split.divvy 4 1 input))

startOfMessage :: String -> Int
startOfMessage input = foldr (\(i, x) acc -> if containsDistincts x then (i + 13) else acc) 0 (zip [1..] (Split.divvy 14 1 input))
