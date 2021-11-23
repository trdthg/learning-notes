import Control.Monad

main = do
    return ()
    -- aaa <- return "aaavvv"
    -- putStrLn aaa

    c <- getChar
    when (c/= ' ') $ do
        putChar c
        main



