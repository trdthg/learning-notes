import System.IO
import System.Directory
import Data.List

main = do
    withFile "todo.txt" ReadMode (\ handle -> do
        contents <- hGetContents handle
        putStr contents
        (tempName, tempHasndle) <- openTempFile "." "todo.txt.tmp"
        let todoTasks = lines contents
            numberedTasks = zipWith (\n line -> show n ++ ". " ++ line) [0..] todoTasks
        putStrLn $ unlines numberedTasks
        putStrLn "which line you want to delete: "
        lineNumber <- getLine
        let number = read lineNumber
            newTodoItems = delete (todoTasks !! number) todoTasks
        hPutStr tempHasndle $ unlines newTodoItems
        hClose tempHasndle
        removeFile "todo.txt"
        renameFile tempName "todo.txt")

