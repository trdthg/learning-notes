import Control.Monad
import Data.Char

main = forever $ do
    putStr "Give a input: "
    l <- getLine
    putStrLn $ map toUpper l
