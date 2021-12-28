

main :: IO ()
main = do
    -- fmap应用于IO
    line <- getLine
    let line' = reverse line
    putStrLn line'
    -- fmap + applicative
    a <- (++) <$> getLine <*> getLine
    putStrLn $ "The two lines concatenated turn out to be: " ++ a