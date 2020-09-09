get_the_streamer_game

# Todo

- [x] Integrate twitchchat wrapper crate
- [x] Interface
  - [x] Title
  - [x] Partition off instruction area
  - [x] Show flame command
  - [x] Show flame graphic
  - [x] Partition off drop zones
  - [x] Label each drop zone (a - z)
- [x] Implement flame on flame command
  - [x] when receiving `#flame-1` spawn a flame entity at position 1
  - [x] Have flame enitity drop
  - [x] Have flame entity stop when it hits the ground
  - [x] Have flame entity dissapear after 5 seconds
- [x] Implement player
  - [x] create player entity + graphic
  - [x] player can move left and right
  - [x] player slows down to a halt when not holding a move key down
  - [x] player can jump
  - [x] player can stand on ground
  - [x] player cannot leave the arena
- [x] game is over after player is hit by an entity spawned by chat
  - [x] display game over text
  - [x] display who won
  - [x] display who participated
- [x] Streamer can win
  - [x] Add a timer
  - [x] If the streamer lives as long as the timer, display win message
  - [x] Fix bad performance around countdown timer
- [ ] Create a second entity that chat can spawn
  - [ ] spin the sword (look at offset [@ootsby])
- Refactor
  - [ ] Create command data struct
  - [ ] abstract redundant command creation code into a function
  - [ ] abstract redundant entity creation code into a function
- [ ] Create a third entity that chat can spawn
- [ ] Create a fourth entity that chat can spawn
- [ ] Polish
  - [ ] When quitting the app, exit all processes
  - [ ] Scale up the fire sprite
  - [ ] Cut the player sprites so that we have left run, right run, and stand
  - [ ] Switch player to have multiple sprites based on which direction they are moving (can mirror sprites by scale using a negative number [from #BareDoodah])
  - [ ] Save a score to disk
    - [ ] 1 point for participating
    - [ ] 10 points for the win
  - [ ] Display text for subscribers over their drops
  - [ ] Stop drops outside of the game, but still on the screen (like in the instructions area)
  - [ ] Add custom winning messages
    - [ ] Victory is Mine!
    - [ ] Who is the Best?
    - [ ] Not today!
- [ ] Clean up code
  - [ ] Make command.rs nicer
  - [ ] Extracting command in lib/update is not great :(
  - [x] Not using children in game objects so remove them
- [ ] Deploy
  - [ ] Create instructions for how to start the game
  - [ ] Compile for Windows
  - [ ] Compile for Mac
  - [ ] Compile for Linux
- [ ] Feature Requests
  - [ ] Blink the teammates names in the game over screen (Chantillycake)
- [ ] Maybe if we have performance issues
  - [ ] Load and store sprites in a data structure to get references later
