import Data.List
import Data.List.Split
import qualified Data.Map as Map

main = do
  content <- readFile "../inputs/day7.txt"
  let input = lines content
  let paths = mapPaths input [] Map.empty
  putStrLn . show . part1 $ paths
  putStrLn . show . part2 paths $ input
  -- putStrLn . show . totalSize $ input
  -- putStrLn . show $ (tail . intercalate "/" . reverse) $ snd $ changeDirectory (snd (changeDirectory [] "/")) "e"
  -- putStrLn $ show $ parseCommand "$ cd test"

changeDirectory :: [String] -> String -> ([String], [String]) 
changeDirectory path target = case target of
  ".." -> (path, tail path)
  _ -> (path, target : path)

parseFile :: String -> (Int, String)
parseFile line =
  let (size, name) = span (/=' ') line
   in (read size, name)

parseDirectory :: String -> String 
parseDirectory = head . words

parseCommand :: String -> String
parseCommand = last . words

isDir :: String -> Bool
isDir = (=="dir") . take 3

isCd :: String -> Bool
isCd = (=="$ cd") . take 4

isLs :: String -> Bool
isLs = (== "$ ls")

parsePath :: [String] -> String
parsePath = tail . intercalate "/" . reverse

mapPaths :: [String] -> [String] -> Map.Map String Int -> Map.Map String Int
mapPaths input path paths = case input of
  [] -> paths
  x : xs | isCd x ->  let dir = parseCommand x
                          (oldPath, newPath) = changeDirectory path dir
                          size = Map.findWithDefault 0 (parsePath oldPath) paths in
                            mapPaths xs newPath (if dir == ".." then (Map.insertWith (+) (parsePath newPath) size paths) else paths)
         | isLs x -> mapPaths xs path paths
         | isDir x -> mapPaths xs path paths
         | otherwise -> let (size, name) = parseFile x in
                            mapPaths xs path $ Map.insertWith (+) (parsePath path) size paths  

part1 :: Map.Map String Int -> Int
part1 paths = Map.foldr (+) 0 $ Map.filter (< 100000) paths

totalSize :: [String] -> Int
totalSize lines = foldl (\acc x -> if not (isCd x) && not (isLs x) && not (isDir x) then acc + fst (parseFile x) else acc) 0 lines

toBeDeleted :: [String] -> Int
toBeDeleted lines = 30000000 - (70000000 - totalSize lines)

part2 :: Map.Map String Int -> [String] -> Int
part2 paths lines = head . dropWhile (<= toBeDeleted lines) . sort . snd . unzip . Map.toList $ Map.delete "" paths
