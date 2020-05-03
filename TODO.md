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
    - [ ] RRIK pseudo inverse method iterative solver
- [ ] NR agent milestone hopping on 2D PRM/A* generated path

# Advanced agent
- [ ] Angle bound obstacles, replanning in known environment
- [ ] Prismatic joints
- [ ] Human link agent
- [ ] Energy spent in some form
- [ ] Gravity
- [ ] Springy limbs

# Effects
- [ ] Belay rope
- [ ] Rock particle effects while holding on
- [ ] Wind effects

# Environment
- [ ] Spherical obstacles, line segment obstacles
- [ ] Slippery holds
- [ ] Unknown environment (note: the milestones can't be sampled they are part of environment)

# Rendering
- [ ] 3D context (holds, trees, waterfall, lavafall, birds)
- [ ] 3D models for agent links (hands, legs, body and tail)?

# Make a game
- [ ] Multiple such agents trying to catch player
- [ ] Shoot down agent from ground

# Data structures
- [ ] KD tree for finding nearest hold
