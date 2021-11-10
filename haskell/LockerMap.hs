import qualified Data.Map as Map

-- declear LockerMap
data LockerState = Taken | Free deriving (Show, Eq)
type Code = String
type LockerMap = Map.Map Int (LockerState, Code)

-- give a LoockerLookUp function
lockerLookUp :: Int -> LockerMap -> Either String Code
lockerLookUp lockerNum map = case Map.lookup lockerNum map of
        Nothing  -> Left $ "Locker Number" ++ show lockerNum ++ "doesn't exist"
        Just (lockerState, code) -> if lockerState == Taken
                                 then Left $ "Locker Number" ++ show lockerNum ++ "is already taken!"
                                 else Right code

main = do
    let lockers = Map.fromList[(1, (Taken, "aaa")),(2, (Free, "bbb")),(3, (Taken, "ccc"))]
    print $ lockerLookUp 1 lockers
    print $ lockerLookUp 2 lockers
    print $ lockerLookUp 3 lockers
    print $ lockerLookUp 4 lockers
