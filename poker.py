import random

def winner(deal):
    hand = poker(deal)
    cards = []

    for x in hand:
        if x > 0 and x < 14:
            cards.append(str(x)+"C")
        if x > 13 and x < 27:
            cards.append(str(1+x%13)+"D")
        if x > 26 and x < 40:
            cards.append(str(1+x%13)+"H")
        if x > 39 and x < 53:
            cards.append(str(1+x%13)+"S")
    print(cards)

def poker(deal):
    hand1 = [deal[i] for i in range(0, 10, 2)]
    hand2 = [deal[i] for i in range(1, 10, 2)]

    print("Deal:", deal)
    print("Hand 1:", hand1)
    print("Hand 2:", hand2)

    royalFlush1 = royalFlush(hand1)
    royalFlush2 = royalFlush(hand2)

    straightFlush1 = straightFlush(hand1)
    straightFlush2 = straightFlush(hand2)

    fourKind1 = fourKind(hand1)
    fourKind2 = fourKind(hand2)

    fullHouse1 = fullHouse(hand1)
    fullHouse2 = fullHouse(hand2)

    flush1 = flush(hand1)
    flush2 = flush(hand2)

    straight1 = straight(hand1)
    straight2 = straight(hand2)

    threeKind1 = threeKind(hand1)
    threeKind2 = threeKind(hand2)

    twoPairs1 = twoPairs(hand1)
    twoPairs2 = twoPairs(hand2)

    pairs1 = pairs(hand1)
    pairs2 = pairs(hand2)

    if royalFlush1 or royalFlush2:
        if royalFlush1 and royalFlush2:return highCard(hand1, hand2)
        if royalFlush1:return hand1
        else:return hand2

    elif straightFlush1 or straightFlush2:
        if straightFlush1 and straightFlush2:return highCard(hand1, hand2)
        if straightFlush1:return hand1
        else:return hand2

    elif fourKind1 or fourKind2:
        if fourKind1 and fourKind2:return highCard(hand1, hand2)
        if fourKind1:return hand1
        else:return hand2

    elif fullHouse1 or fullHouse2:
        if fullHouse1 and fullHouse2:return highCard(hand1, hand2)
        if fullHouse1:return hand1
        else:return hand2

    elif flush1 or flush2:
        if flush1 and flush2:return highCard(hand1, hand2)
        if flush1:return hand1
        else:return hand2

    elif straight1 or straight2:
        if straight1 and straight2:return highCard(hand1, hand2)
        if straight1:return hand1
        else:return hand2

    elif threeKind1 or threeKind2:
        if threeKind1 and threeKind2:return highCard(hand1, hand2)
        if threeKind1:return hand1
        else:return hand2

    elif twoPairs1 or twoPairs2:
        if twoPairs1 and twoPairs2:return highCard(hand1, hand2)
        if twoPairs1:return hand1
        else:return hand2

    elif pairs1 or pairs2:
        if pairs1 and pairs2:return highCard(hand1, hand2)
        if pairs1:return hand1
        else:return hand2

    else:
        return highCard(hand1, hand2)

def sortCards(hand1, hand2):
    hand1.sort()
    hand2.sort()

def royalFlush(hand):
    royals = [[1, 10, 11, 12, 13], [i+13 for i in [1, 10, 11, 12, 13]], [i+26 for i in [1, 10, 11, 12, 13]], [i+39 for i in [1, 10, 11, 12, 13]]]
    if hand in royals: return True
    return False

def straightFlush(hand):
    for x in range(0, 4):
        if (hand[x+1] - hand[x]) != 1:
            return False
    return True

def fourKind(hand):
    kind = rank(hand)
    if kind.count(kind[0]) == 4 or kind.count((kind[-1])) == 4: return True
    return False

def fullHouse(hand):
    kind = rank(hand)
    if (kind.count(kind[0]) == 2 and kind.count((kind[-1])) == 3) or (kind.count(kind[0]) == 3 and kind.count((kind[-1])) == 2): return True
    return False

def flush(hand):
    suits = [[i for i in range(1, 14)], [i+13 for i in range(1, 14)], [i+26 for i in range(1, 14)], [i+39 for i in range(1, 14)]]
    for x in suits:
        if set(hand).issubset(set(x)):
            return True
    else:return False

def straight(hand):
    kind = rank(hand)
    for i in range(0, 4):
        if kind[i+1] - kind[i] != -1:return False
    else:return True

def threeKind(hand):
    kind = rank(hand)
    if kind.count(kind[2]) == 3:return True
    return False

def twoPairs(hand):
    kind = rank(hand)
    ret = False
    if kind.count(kind[0]) == 2:
        rem = kind[0]
        kind.remove(rem)
        kind.remove(rem)
        if kind.count(kind[0])== 2 or kind.count(kind[2]) == 2:ret = True
    elif kind.count(kind[2]) == 2:
        rem = kind[2]
        kind.remove(rem)
        kind.remove(rem)
        if kind.count(kind[0])== 2 or kind.count(kind[2]) == 2:ret = True
    elif kind.count(kind[4]) == 2:
        rem = kind[4]
        kind.remove(rem)
        kind.remove(rem)
        if kind.count(kind[0])== 2 or kind.count(kind[2]) == 2:ret = True
    return ret

def pairs(hand):
    kind = rank(hand)
    if kind.count(kind[0]) == 2 or kind.count(kind[2]) == 2 or kind.count(kind[4]) == 2:return True
    return False

def highCard(hand1, hand2):
    kind1 = rank(hand1)
    kind2 = rank(hand2)
    for x in range(0, 5):
        if kind1[x] != kind2[x]:
            if kind1[x] > kind2[x]:
                return hand1
            else:
                return hand2
    else:
        return best(hand1, hand2)

def best(hand1, hand2):
    if sum(hand1)>sum(hand2):return hand1
    else:return hand2

def rank(hand):
    kind = [i%13 for i in hand]
    kind.sort(reverse=True)
    return kind


for x in range(1):
    print("Test", x)
    winner([random.randrange(1, 53) for i in range(1, 11)])