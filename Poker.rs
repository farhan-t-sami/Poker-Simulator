// Name: Farhan Sami 
// RSN: 500892031

pub fn deal(cards:[u32;10]) -> [String;5] {
    let mut hand1:[u32;5] = [cards[0],cards[2],cards[4],cards[6],cards[8]];
    let mut hand2:[u32;5] = [cards[1],cards[3],cards[5],cards[7],cards[9]];

    hand1.sort();
    hand1.reverse();
    hand2.sort();
    hand2.reverse();

    let royalFlush1:bool = royalFlush(hand1);
    let royalFlush2:bool = royalFlush(hand2);

    let straightFlush1:bool = straightFlush(hand1);
    let straightFlush2:bool = straightFlush(hand2);

    let fourKind1:bool = fourKind(hand1);
    let fourKind2:bool = fourKind(hand2);

    let fullHouse1:bool = fullHouse(hand1);
    let fullHouse2:bool = fullHouse(hand2);

    let flush1:bool = flush(hand1);
    let flush2:bool = flush(hand2);

    let straight1:bool = straight(hand1);
    let straight2:bool = straight(hand2);

    let threeKind1:bool = threeKind(hand1);
    let threeKind2:bool = threeKind(hand2);

    let twoPairs1:bool = twoPairs(hand1);
    let twoPairs2:bool = twoPairs(hand2);

    let pairs1:bool = pairs(hand1);
    let pairs2:bool = pairs(hand2);

    let mut winner:[u32;5] = [0,0,0,0,0];

    if royalFlush1 || royalFlush2 
    {
        if royalFlush1 && royalFlush2
        {
            winner = highCard(hand1, hand2);
        }
        else if royalFlush1 && !royalFlush2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if straightFlush1 || straightFlush2 
    {
        if straightFlush1 && straightFlush2
        {
            winner = highCard(hand1, hand2);
        }
        else if straightFlush1 && !straightFlush2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if fourKind1 || fourKind2 
    {
        if fourKind1 && fourKind2
        {
            winner = highCard(hand1, hand2);
        }
        else if fourKind1 && !fourKind2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if fullHouse1 || fullHouse2 
    {
        if fullHouse1 && fullHouse2
        {
            winner = highCard(hand1, hand2);
        }
        else if fullHouse1 && !fullHouse2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if flush1 || flush2 
    {
        if flush1 && flush2
        {
            winner = highCard(hand1, hand2);
        }
        else if flush1 && !flush2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if straight1 || straight2 
    {
        if straight1 && straight2
        {
            winner = highCard(hand1, hand2);
        }
        else if straight1 && !straight2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if threeKind1 || threeKind2 
    {
        if threeKind1 && threeKind2
        {
            winner = highCard(hand1, hand2);
        }
        else if threeKind1 && !threeKind2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if twoPairs1 || twoPairs2 
    {
        if twoPairs1 && twoPairs2
        {
            winner = highCard(hand1, hand2);
        }
        else if twoPairs1 && !twoPairs2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }
    else if pairs1 || pairs2 
    {
        if pairs1 && pairs2
        {
            winner = highCard(hand1, hand2);
        }
        else if pairs1 && !pairs2
        {
            winner = hand1;
        }
        else
        {
            winner = hand2;
        }
    }

    else 
    {
        winner = highCard(hand1, hand2);
    }

    let mut answer:[String;5] = [String::from(""), String::from(""), String::from(""), String::from(""), String::from("")];

    for card in 0..5
    {
        let x = winner[card];

        if x > 0 && x < 11
        {
            let mut s:String = x.to_string();
            s.push('C');
            answer[card] = s;
        }

        else if x > 13 && x < 24
        {
            let mut s:String = x.to_string();
            s.push('D');
            answer[card] = s;
        }

        else if x > 26 && x < 37
        {
            let mut s:String = x.to_string();
            s.push('H');
            answer[card] = s;
        }

        else if x > 39 && x < 50
        {
            let mut s:String = x.to_string();
            s.push('S');
            answer[card] = s;
        }

        else
        {
            let s:String = x.to_string();
            answer[card] = s;
        }
    }

    answer.reverse();
    answer
}

pub fn royalFlush(hand: [u32;5]) -> bool {
    let royals1: [u32; 5] = [1,10,11,12,13];
    let royals2: [u32; 5] = [14,23,24,25,26];
    let royals3: [u32; 5] = [27,36,37,38,39];
    let royals4: [u32; 5] = [40,49,50,51,52];
    let mut ret: bool = true;
    let mut ord:[u32;5] = hand;
    ord.sort();

    if !((ord == royals1) || (ord == royals2) || (ord == royals3) || (ord == royals4)) {
        ret = false;
    }
    ret
}

pub fn straightFlush(hand: [u32;5]) -> bool {
    flush(hand) && straight(hand)
}

pub fn fourKind(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let kind:[u32;5] = rank(hand);
    if (kind[0]==kind[1] && kind[1]==kind[2] && kind[2]==kind[3]) || (kind[4]==kind[1] && kind[1]==kind[2] && kind[2]==kind[3]) {ret = true;}
    ret
}

pub fn fullHouse(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let kind:[u32;5] = rank(hand);
    if ( kind[0] == kind[1] && kind[2] == kind [3] && kind[3] == kind[4] ) || ( kind[0] == kind[1] && kind[1] == kind [2] && kind[3] == kind[4] ) {ret = true;}
    ret
}

pub fn flush(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let set1:[u32;13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let set2:[u32;13] = [14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];
    let set3:[u32;13] = [27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39];
    let set4:[u32;13] = [40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52];
    
    if (set1.contains(&hand[0]) && set1.contains(&hand[1]) && set1.contains(&hand[2]) && set1.contains(&hand[3]) && set1.contains(&hand[4])) || 
    
    (set2.contains(&hand[0]) && set2.contains(&hand[1]) && set2.contains(&hand[2]) && set2.contains(&hand[3]) && set2.contains(&hand[4])) || 
    
    (set3.contains(&hand[0]) && set3.contains(&hand[1]) && set3.contains(&hand[2]) && set3.contains(&hand[3]) && set3.contains(&hand[4])) || 
    
    (set4.contains(&hand[0]) && set4.contains(&hand[1]) && set4.contains(&hand[2]) && set4.contains(&hand[3]) && set4.contains(&hand[4])) {ret = true;}
    
    ret
}

pub fn straight(hand: [u32;5]) -> bool {
    let mut ret: bool = true;
    let mut ord:[u32;5] = rank(hand);
    ord.sort();
    for x in 0..4 {
        if ord[x] + 1 != ord[x+1]{
            ret = false;
            break;
        }
    }
    ret
}

pub fn threeKind(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let kind:[u32;5] = rank(hand);
    if (kind[2] == kind [0] && kind[2] == kind[1]) || (kind[2] == kind [3] && kind[2] == kind[4]) {ret = true;}
    ret
}

pub fn twoPairs(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let kind:[u32;5] = rank(hand);
    let mut ind0 = 0;
    let mut ind1 = 0;
    let mut ind2 = 0;
    let mut ind3 = 0;
    let mut ind4 = 0;
    let couple = 2;
    for x in 0..5 {
        if kind[x] == kind[0] {ind0 = ind0+1;}
        if kind[x] == kind[1] {ind1 = ind1+1;}
        if kind[x] == kind[2] {ind2 = ind2+1;}
        if kind[x] == kind[3] {ind3 = ind3+1;}
        if kind[x] == kind[4] {ind4 = ind4+1;}
    }
    if  (ind0==couple && ((kind[0] != kind[1] && ind1==couple) || ind2==couple || ind3==couple || ind4==couple)) ||

        (ind1==couple && ((kind[0]!=kind[1] && ind0==couple) || (kind[2]!=kind[1] && ind2==couple) || ind3==couple || ind4==couple)) ||

        (ind2==couple && ((kind[1]!=kind[2] && ind1==couple) || (kind[3]!=kind[2] && kind[3]==couple) || ind0==couple || ind4==couple)) ||

        (ind3==couple && ((kind[2]!=kind[3] && ind2==couple) || (kind[4]!=kind[3] && ind4==couple) || ind0==couple || ind1==couple)) ||

        (ind4==couple && ((kind[4] != kind[3] && ind3==couple) || ind1==couple || ind2==couple || ind3==couple))

    {ret = true;}
    ret
}

pub fn pairs(hand: [u32;5]) -> bool {
    let mut ret:bool = false;
    let kind:[u32;5] = rank(hand);
    let mut ind0 = 0;
    let mut ind2 = 0;
    let mut ind4 = 0;
    for x in 0..5 {
        if kind[x] == kind[0] {ind0 = ind0+1;}
        if kind[x] == kind[2] {ind2 = ind2+1;}
        if kind[x] == kind[4] {ind4 = ind4+1;}
    }
    if ind0 == 2 || ind2 == 2 || ind4 == 2 {ret = true;}
    ret
}

pub fn highCard(hand1:[u32;5], hand2:[u32;5]) -> [u32;5] {
    let kind1:[u32;5] = rank(hand1);
    let kind2:[u32;5] = rank(hand2);
    let mut ret:[u32;5] = best(hand1, hand2);
    for x in 0..5
    {
        if kind1[x] != kind2[x]
        {
            if kind1[x]>kind2[x]
            {   
                ret = hand1;
                break;
            }
            ret = hand2;
            break;
        }
    }
    ret
}

pub fn best(hand1:[u32;5], hand2:[u32;5]) -> [u32;5] {
    let total1: u32 = hand1.iter().sum();
    let total2: u32 = hand2.iter().sum();
    let mut ret:[u32;5] = [0,0,0,0,0];
    if total1 > total2 
    {
        ret = hand1;
    }
    ret = hand2;
    ret
}

pub fn rank(hand: [u32;5]) -> [u32;5]{
    let mut kind: [u32;5] = [0,0,0,0,0];
    for x in 0..5{
        let mut val:u32 = hand[x] % 13;
        if val == 0 {val = 13;}
        kind[x] = val;
    }
    kind.sort();
    kind.reverse();
    kind
}