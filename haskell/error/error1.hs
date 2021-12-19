import System.Environment
import System.IO
import System.Directory
import System.Environment

main = do
    (filename:_) <- getArgs
    fileExists <- doesFileExist filename
    if fileExists
        then do
            contents <- readFile filename
            print . length . lines $ contents
        else print "no this file"
