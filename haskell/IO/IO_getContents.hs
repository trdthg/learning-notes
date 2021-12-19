import Data.Char

main = do
  contents <- getContents
  putStrLn "----- Upper case -----"
  putStrLn $ map toUpper contents
  putStrLn "----- Only short -----"
  putStrLn $ shortLinesOnly contents

shortLinesOnly :: String -> String
shortLinesOnly input =
  let allLines = lines input
      shortLines = filter (\line -> length line < 10) allLines
      result = unlines shortLines
   in result

-- putStrLn "----- interact works too -----"
-- -- interact shortLinesOnly
