# Assumptions
- 2D wall and 2D constrained stick-figure agents
- 2D circle and line segment obstacles on wall

# Stick-figure agent
- A stick-figure agent = rigid links + joints
- Joint = revolute

# Serial stick-figure agent
- Links arranged serially
- N links => N joints => N sized joint tuple => N + 1 ends
- Always one pivot end and one free end

# Basics
- [ ] 2D PRM/A*
    - [ ] Avoid duplication of same start and finish
- [x] Spherical agent translating on 2D PRM/A* generated path
- [x] RR agent milestone hopping on 2D PRM/A* generated path
    - [x] min/max bounds on PRM neighbours
    - [x] Control joint tuple and goal joint tuple ranges
    - [x] RRIK Jacobian transpose iterative solver
    - [x] RRIK pseudo inverse method iterative solver
- [x] NR agent milestone hopping on 2D PRM/A* generated path
    - [x] Jacobian method
    - [x] Pseudo inverse method

# Humanize
- [x] Two arm agent
- [x] Gravity effect
- [x] Two arm Two leg agent
- [ ] Energy spent in some form
- [ ] Worm agent

# Obstacles
- [x] Spherical obstacles, line segment obstacles
- [x] Slippery holds
- [x] Re planning on obstacle collision or slipping

# Effects
- [x] Sound on pivot change
- [ ] Rock particle effects while holding on
- [ ] Waterfall
- [ ] Trail of hand (useful for showing shortest path of pseudo inverse vs jacobian method)
- [ ] Wind effects - both in body, particles

# Rendering
- [ ] 3D context (holds, trees, waterfall, lavafall, birds) and sounds
- [ ] 3D models for agent links (hands, legs, body and tail)?

# Make a game
- [ ] Multiple such agents trying to catch player
- [ ] Shoot down agent from ground

# Demos
- [ ] Better path creations
- [ ] RRAnalytical demo
- [ ] NRIterative agent demos
    - [ ] Jacobian vs Pseudo inverse
- [ ] Two arm agent demo
    - [ ] Climbing up-left, up-right
    - [ ] Climbing down-left, down-right
    - [ ] Climbing sideways
    - [ ] Climbing all in same path

# Future work
- [ ] Prismatic joints
- [ ] Angle bound obstacles, replanning in known environment
- [ ] Belay rope
- [ ] Springy limbs
- [ ] Spatial data structures
- [ ] Unknown environment (note: the milestones can't be sampled they are part of environment)
