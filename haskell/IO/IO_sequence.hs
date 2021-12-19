main = do
  -- 1.1 sequence can run erery IO action in it, and return their result as a list
  -- rs <- sequence [getLine, getLine, getLine]
  -- print rs

  -- 1.2
  -- map print [1, 2, 3]
  -- [print 1, print 2, print 3]
  res <- sequence $ map print [1, 2, 3]
  -- `print 1` just return a `()`
  -- so res is [(), (), ()]
  print res

  -- 2.1 mapM can be used to map a function that return IO action
  mapm <- mapM print [1, 2, 3]
  print mapm
  -- 2.2 mapM_ just abundon the results
  mapm_ <- mapM_ print [1, 2, 3]
  print mapm_
