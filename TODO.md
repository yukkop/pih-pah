[ ] camera
    [x] rotation on mouse
    [ ] restrict movement throw terrain
[x] grab mouse in game
[ ] player respawn
    [ ] delay on respawn
        [ ] store player preferences during respawn
        [ ] death screen
[ ] ingame chat
    [ ] chat history
    [ ] chat colors
[ ] lobby player list
[ ] settings
    [ ] key bindings
    [ ] sensetivity
[ ] nickname above character
[ ] projectile
    [x] collision
    [x] speed
    [x] lifetime
    [x] ballistics
    [x] spawn point
    [ ] spawn delay
    [ ] spawn sound
[ ] terrain
    [x] collision
    [ ] skybox
    [ ] portals
[ ] gravity
    [x] own gravity for any actor
    [ ] gravity zones
    [x] option toggle gravity for character
[ ] map
    [ ] option load custom map
    [ ] transfer map from server to client
[ ] message on port exist in host create

# Tool
[ ] extend commands for chid builder

# Bug
[x] wrong position on player spawn
    [x] loader do not work on changing map (need rewrite it all)
[ ] do not get mass from glb in processes scene system
[x] mouse disapired on esc in menu
[x] widow do not hide after game menu close
[ ] intuitiv mouse position in game menu (I mean that mouse position in game menu shold be on last position or in center of screen) 
[ ] if open game menu during camera movement it will be spin ever
[x] viewport size do not update after window size hase been changed
[x] game menu do not use debug viewport for size
[x] gravity layers broken
[ ] random orders in game systems
[ ] Camera raycast must ignore projectile
[ ] just presed realisation for multiplayr inputs

# Critical
[ ] Insert, Spawn etc. add commands into command list that have non-obvious order. need to change it in parts where order important