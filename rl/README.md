### Assumptions
- 2D wall and 2D constrained stick-figure agents
- 2D circle and line segment obstacles on wall

### Stick-figure agent
- A stick-figure agent = rigid links + joints
- Joint = revolute

### Serial stick-figure agent
- Links arranged serially
- N links => N joints => N sized joint tuple => N + 1 ends
- Always one pivot end and one free end

### Basics
- [x] Spherical agent translating on 2D PRM/A* generated path
- [x] RR agent milestone hopping on 2D PRM/A* generated path
    - [x] min/max bounds on PRM neighbours
    - [x] Control joint tuple and goal joint tuple ranges
    - [x] RRIK Jacobian transpose iterative solver
    - [x] RRIK pseudo inverse method iterative solver
- [x] NR agent milestone hopping on 2D PRM/A\* generated path
    - [x] Jacobian method
    - [x] Pseudo inverse method

### Humanize
- [x] Two arm agent
- [x] Gravity effect
- [x] Two arm Two leg agent
- [x] Energy spent in some form

### Obstacles
- [x] Spherical obstacles, line segment obstacles
- [x] Slippery holds and rock particle effects
- [x] Re planning on obstacle collision or slipping

### Effects
- [x] Sound
  - [x] On pivot change
  - [x] On slip
  - [x] Environment
- [x] Waterfall
- [x] Wind effects - both in body, particles

### Rendering
- [x] 3D context (holds, trees, waterfall, lavafall, birds) and sounds

### Polishing
- [x] Improve leaf effects especially in with context demo
- [x] Color according to energy, remove energy bar
- [x] Competition b/w agents with slippery holds
- [x] Tune params for Four arm agent demos

- [x] Change ground doesn't go with everything else; make it seem like agent is high above the ground
- [x] Add obstacles
- [x] Improve agent rendering, 3D models for agent links (hands, legs, body and tail)?

- [x] Trail of hand (useful for showing shortest path of pseudo inverse vs jacobian method)
- [x] Worm agent
- [x] Improve colors of water and sky

### Demos
- [x] RRAnalytical demo
- [x] NRIterative agent demos
    - [x] Jacobian vs Pseudo inverse
- [x] Two arm agent demo
    - [x] Climbing up-left, up-right
    - [x] Climbing down-left, down-right
    - [x] Climbing sideways
    - [x] Climbing all in same path
- [x] A demo containing everything

### Future work
- [ ] JT agent vs CEO agent
- [ ] Constrined q: JT agent vs CEO agent
- [ ] 2D PRM/A\*
    - [ ] Avoid duplication of same start and finish
- [ ] Belay rope
- [ ] Angle bound obstacles, replanning in known environment
- [ ] Prismatic joints
- [ ] Springy limbs
- [ ] Spatial data structures
- [ ] Unknown environment (note: the milestones can't be sampled they are part of environment)
