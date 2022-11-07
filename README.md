# Big Two

## Game Loop

### Assumptions
- 52 standard playing cards
- 4 Players, start with 13 cards each
- First player to have 0 cards wins

### Initialization

- 52 Shuffled Cards dealt to 4 Players, 13 each
- Player who has the Three of Clubs [3C] is first

### Starting a Trick

- [First Trick Only] Player with the Three of Clubs starts, they must play a valid hand that contains the Three of Clubs.
- [All Other Tricks] Player who won the previous Trick gets to start, with any valid hand that they can make with their hand.
- Next Player is Counter-Clockwise.

### Trick Loop

- Current Player either:
  - Plays a valid hand from their cards, that "beats" (highest card of played hand is higher than highest card of previous hand).
    - If these are the Player's last cards, Gamw Over and Player Wins.
  - Passes, plays no cards and exits the Trick.
    - If this is the 3rd player to Exit the Trick, i.e. there is only one other player remaining in the Trick, then the Trick is over, last remaining player in Trick wins the Trick
- Counter-Clockwise to next Player (who has not already passed).

## Current Game Limitations (i.e. TODOs)
- Only supports Lone, Pairs, and Trips
  - Need to implement Fivers: Straight, Flush, FourPlusKick, StraightFlush
  - Need to implement Bombs
- NPC AI only plays Lone, will pass on Pairs and Trips
  - Need to implement AI that can play on anything 

## Current Code Organization Issues
- main contains the logis to initialize the game and run Tricks
  - Need to make a Game module and relocate this logic (shuffle deck, deal cards, find player with 3C)

