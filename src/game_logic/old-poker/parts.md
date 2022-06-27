
# Player
Trait for a player

## Functions
* Make bet (perform an action)
    * Needs information about the table. Players, flop, pot_size
    * Needs to know hand
    * To make life easier, how much required to call

# Player Interface
* This is what the table sees AND every other player. It is how the table interacts with the player
* Immutable reference of TABLE is passed to the player

## Variables
* Pointer to player
* Chips
    * Getter
* Hand
* Chips in pot
    * Getter
* Last performed action
    * Getter

## Functions
* Make bet
    * Calls the player's bet function
* 


