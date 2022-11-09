# Big Two

[A card game of Cantonese origin](https://en.wikipedia.org/wiki/Big_two), played by children
and adults alike across the world, with nothing more than a Standard 52 Card Deck.

## Game Rules

Please keep in mind, Big Two has many variations, this is merely one interpretation.

### Objective 
- Take turns playing cards, following the rules for valid combos
- First player to have 0 cards wins!

### Initialization

- 52 Shuffled Cards dealt to 4 Players, 13 each
- Player who has the Three of Clubs is first

### Starting a Trick

- **First Trick Only**: Player with the Three of Clubs starts, they must play a valid hand that contains the Three of Clubs.
- **All Other Tricks**: Player who won the previous Trick gets to start, with any valid hand that they can make with their hand.
- Next Player is Counter-Clockwise.

### Trick Loop

- Current Player either:
    - Plays a valid hand from their cards, that ["beats"](#comparing-hands) the previously played hand.
        - If these are the Player's last cards, Gamw Over and Player Wins.
    - Passes, plays no cards and exits the Trick.
        - If this is the 3rd player to Exit the Trick, i.e. there is only one other player remaining in the Trick, then the Trick is over, last remaining player in Trick wins the Trick
- Counter-Clockwise to next Player (who has not already passed).

### Valid Hands

The following are the possuble valid hands, in increasing order of rareness and value.

- Lones (aka Singles, High Card): Just a single Card.
- Pairs (aka Dubs, Two-Of-A-Kind): Two Cards of the same Rank.
- Trips (aka Three-Of-A-Kind): Three Cards of the same Rank.
- (TODO) Fivers (aka Five Card hands):
    - Straight: each Card in the Hand is of consecutive Rank from the previous Card in the Hand.
    - Flush: all 5 Cards are the same Suit.
    - Full-House: a Pair and a Trip.
    - Four-Of-A-Kind-Plus-Kicker: 4 cards of the same Rank, plus any arbitrary additional card.
    - Straigh-Flush: both a Straight and a Flush at the same time.
- (TODO) Bombs
    - Bombs are special combos that may be played on anything except Lones.
        - i.e. They are not restricted to only being played on Hands of the same number of Cards.

### Comparing Hands

When playing a Hand after another player, the incoming Hand must "beat" the previous Hand.

Rank always takes precedence to Suit! E.g. a Seven of Clubs beats a Six of Spades.

- Lones can only be beaten by higher Lones.
  - Bombs CAN NOT be played upon Lones.
- Pairs can only be beaten by higher Pairs or Bombs.
- Trups can only be beaten by higher Trips or Bombs.
- Fivers can be beaten by higher Fivers or Bombs.
    - Between any two different types of Fivers, the higher type always wins.
        - i.e. any Straigh-Flush beats every Four-Of-A-Kind-Plus-Kicker, and so on.
    - between Straights, compare the highest card in each (Rank first, then Suit).
    - between Flushes, compare the Rank of the highest card in each.
    - between Full-Houses, compare the Rank of the Trips.
        - Pairs Rank is disregarded.
    - Between Four-Of-A-Kind-Plus-Kicker, compare Rank of the Four-Of-A-Kind.
        - Kicker's Rank is disregarded.
    - Between Straigh-Flushes, compare the highest card in each (Rank first, then Suit).

## Current Game Limitations (i.e. TODOs)
- Only supports Lone, Pairs, and Trips
    - Need to implement Fivers: Straight, Flush, FourPlusKick, StraightFlush
    - Need to implement Bombs
- NPC AI only plays Lone, will pass on Pairs and Trips
    - Need to implement AI that can play on anything 
- Currently only Single-Player
    - Need to implement Local Multiplayer (pass the keyboard style).
    - Need to implement Cloud Multiplayer ("jackbox style", room with a code).

