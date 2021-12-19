import System.IO

main = do
  handle <- openFile "haiku.txt" ReadMode
  -- Lazy, don't read and stored in memory an once
  contents <- hGetContents handle
  putStrLn contents
  hClose handle

  withFile
    "haiku.txt"
    ReadMode
    ( \handle -> do
        contents <- hGetContents handle
        putStrLn contents
    )

  -- withOpen's define
  -- openFle :: FilePath -> IOMode -> (Handle -> IO a) -> IO a
  -- openFile path mode f = do
  --     handle <- openFile path mode
  --     result <- f handle
  --     hClose f
  --     return result

  -- More function
  -- hGetLine hPutStr hPutStrLn hGetChar
  contents <- readFile "todo.txt"
  putStrLn contents

  todo <- getLine
  appendFile "todo.txt" $ todo ++ "\n"

  -- set buffering manualy
  -- BufferMode = NoBuffering | LineBuffering | BlockBuffering (Maybe Int)
  withFile
    "todo.txt"
    ReadMode
    ( \handle -> do
        hSetBuffering handle $ BlockBuffering (Just 2)
        contents <- hGetContents handle
        putStrLn contents
    )

-- read file in big chunks can help to minimize disk access or when our file is actually a slow network resource
