-- Name: Farhan Sami -- RSN: 500892031
-- Name: Sameer Naumani -- RSN: 500703778

module Poker
(deal) where

import Data.List

-- deal function: produces a list of strings from winner function return
deal cards = do
    let win = winner cards

    let one = head win
    let two = head (tail win)
    let three = head (tail (tail win))
    let four = head (tail (tail (tail win)))
    let five = last win

    let result = suit one : suit two : suit three : suit four : suit five :[]

    result

-- executes all conditions and decides winner using the Poker rule logic
winner cards = do
    let hand1 = sort (evens cards)
    let hand2 = sort (odds cards)

    let royalFlush1 = royalFlush hand1
    let royalFlush2 = royalFlush hand2

    let straightFlush1 = straightFlush hand1
    let straightFlush2 = straightFlush hand2

    let fourKind1 = fourKind hand1
    let fourKind2 = fourKind hand2

    let fullHouse1 = fullHouse hand1
    let fullHouse2 = fullHouse hand2

    let flush1 = flush hand1
    let flush2 = flush hand2

    let straight1 = straight hand1
    let straight2 = straight hand2

    let threeKind1 = threeKind hand1
    let threeKind2 = threeKind hand2

    let twoPairs1 = twoPairs hand1
    let twoPairs2 = twoPairs hand2

    let pairs1 = pairs hand1
    let pairs2 = pairs hand2

    if royalFlush1 || royalFlush2
    then if royalFlush1 && royalFlush2
        then highCard hand1 hand2
        else if royalFlush1
            then hand1
            else hand2

    else if straightFlush1 || straightFlush2
        then  if straightFlush1 && straightFlush2
            then highCard hand1 hand2
            else if straightFlush1
                then hand1
                else hand2

        else if fourKind1 || fourKind2
                then if fourKind1 && fourKind2
                    then highCard hand1 hand2
                    else if fourKind1
                        then hand1
                        else hand2

            else if fullHouse1 || fullHouse2
                    then if fullHouse1 && fullHouse2
                        then highCard hand1 hand2
                        else if fullHouse1
                            then hand1
                            else hand2

                else if flush1 || flush2
                        then if flush1 && flush2
                            then highCard hand1 hand2
                            else if flush1
                                then hand1
                                else hand2

                    else if straight1 || straight2
                            then if straight1 && straight2
                                then highCard hand1 hand2
                                else if straight1
                                    then hand1
                                    else hand2

                        else if threeKind1 || threeKind2
                                then if threeKind1 && threeKind2
                                    then highCard hand1 hand2
                                    else if threeKind1
                                        then hand1
                                        else hand2

                            else if twoPairs1 || twoPairs2
                                    then if twoPairs1 && twoPairs2
                                        then highCard hand1 hand2
                                        else if twoPairs1
                                            then hand1
                                            else hand2

                                else if pairs1 || pairs2
                                        then if pairs1 && pairs2
                                            then highCard hand1 hand2
                                            else if pairs1
                                                then hand1
                                                else hand2 

                                    else highCard hand1 hand2

-- return true if hand is a royal flush
royalFlush :: [Integer] -> Bool
royalFlush hand
    | hand == [1,10,11,12,13] = True
    | hand == [14,23,24,25,26] = True
    | hand == [27,36,37,38,39] = True
    | hand == [40,49,50,51,52] = True
    | otherwise = False

-- return true if hand is a straight flush
straightFlush hand = flush hand && straight hand

-- return true if hand is a four kind
fourKind hand = do
    let kind = rank hand
    let one = head kind
    let two = head (tail kind)
    let three = head (tail (tail kind))
    let four = head (tail (tail (tail kind)))
    let five = last kind
    if (one == two && two == three && three == four) || (five == two && two == three && three == four)
    then True
    else False    

-- return true if hand is a full house
fullHouse hand = do
    let kind = rank hand
    let one = head kind
    let two = head (tail kind)
    let three = head (tail (tail kind))
    let four = head (tail (tail (tail kind)))
    let five = last kind
    if (one == two && three == four && three == five) || (one == two && one == three && four == five)
    then True
    else False

-- return true if hand is a flush
flush :: [Integer] -> Bool
flush hand
    | subList hand [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13] == True = True
    | subList hand [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26] == True = True
    | subList hand [27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39] == True = True
    | subList hand [40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52] == True = True
    | otherwise = False

-- return true if hand is a straight
straight hand = do
    let kind = rank hand
    let first = head kind
    let last = first + 4
    let straight = [x | x <- [first..last]]
    if kind == straight
    then True
    else False

-- return true if hand is a three kind
threeKind hand = do
    let kind = rank hand
    let middle = head (tail (tail kind))
    if subList [middle, middle, middle] kind
    then True
    else False 

-- return true if hand is a two pair
twoPairs hand = do
    let kind = rank hand
    let one = head kind
    let two = head (tail kind)
    let three = head (tail (tail kind))
    let four = head (tail (tail (tail kind)))
    let five = last kind
    if (count one kind == 2 && ((count two kind == 2 && one /= two) || count three kind == 2 || count four kind == 2 || count five kind == 2) || count two kind == 2 && ((count one kind == 2 && one /= two) || (count three kind == 2 && two /= three) || count four kind == 2 || count five kind == 2) || count three kind == 2 && ((count two kind == 2 && two /= three) || count one kind == 2 || (count four kind == 2 && three /= four) || count five kind == 2) || count four kind == 2 && (count two kind == 2 || (count three kind == 2 && three /= four) || count one kind == 2 || (count five kind == 2 && four /= five)) || count five kind == 2 && (count two kind == 2 || count three kind == 2 || (count four kind == 2 && four /= five) || count one kind == 2))
    then True
    else False

-- return true if hand is a pair
pairs hand = do
    let kind = rank hand
    let one = head kind
    let three = head (tail (tail kind))
    let five = last kind
    if count one kind == 2 || count three kind == 2 || count five kind == 2
    then True
    else False

-- returns the higher of two hands
highCard hand1 hand2 = do
    let kind1 = rank hand1
    let kind2 = rank hand2
    if hand1 == hand2
    then best hand1 hand2
    else if hand1 > hand2
        then hand1
        else hand2

-- returns the hand with cards from higher suits
best hand1 hand2 = do
    if sum hand1 > sum hand2
    then hand1
    else hand2

-- converts all suits to single rank
rank hand = sort ([x `mod` 13 | x <- hand])

-- converts the number to rank and returns it as string
rankstr num = show (num `mod` 13)

-- returns the letter for suit that the card belongs to
suit :: Integer -> String
suit card
    | card == 11 = "Jack C" 
    | card == 12 = "Queen C"
    | card == 13 = "King C"
    | card == 24 = "Jack D"  
    | card == 25 = "Queen D" 
    | card == 26 = "King D" 
    | card == 37 = "Jack H"  
    | card == 38 = "Queen H" 
    | card == 39 = "King H" 
    | card == 50 = "Jack S"  
    | card == 51 = "Queen S" 
    | card == 52 = "King S" 
    | card > 0 && card < 14 = rankstr card ++ "C"
    | card > 13 && card < 27 = rankstr card ++ "D"
    | card > 26 && card < 40 = rankstr card ++ "H"
    | card > 39 && card < 53 = rankstr card ++ "S"
    | otherwise = "invalid card"

-- checks if a list is sub list of another function
subList :: Eq a => [a] -> [a] -> Bool
subList [] [] = True
subList _ []    = False
subList [] _    = True
subList (x:xs) (y:ys) 
    | x == y    = subList xs ys   
    | otherwise = subList (x:xs) ys

-- counts the number of occurences of the element in list
count   :: Eq a => a -> [a] -> Int
count x =  length . filter (==x)

-- splits list into lists of even and odd indices
evens (x:xs) = x:odds xs
evens _ = []
odds (_:xs) = evens xs
odds _ = []