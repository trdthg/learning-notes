import Control.Monad

main = do
    colors <- forM [1,2,3] (\a -> do
        l <- getLine
        return $ show a ++ ": " ++ l)
    mapM putStrLn colors
