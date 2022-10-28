# Big Two

## Game Loop

### Assumptions

- 4 Players
- Game Over when any Player plays their last card(s)
- Remaining Player's points added up (lower is better)

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

