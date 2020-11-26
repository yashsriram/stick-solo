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
- [x] Worm
- [x] Two arm agent demo
- [x] Four arm agent demo
    - [x] Climbing up-left, up-right
    - [x] Climbing down-left, down-right
    - [x] Climbing sideways
    - [x] Climbing all in same path
- [x] Four arm agent race demo
- [x] A demo containing everything

### New ideas
- Idea for human like stick figure agent.
    - [ ] IK for each limb.
    - [ ] Co-ordinate among limbs.
    - [ ] With q contrains (including one side leg goal only, no cross overs).
    - [ ] Put center of mass over holds.
    - [ ] Formulate and achieve relaxing poses/efficient transfers.
    - [ ] Do what all climb cycle agent can and more (ex same hand two time transfer).
    - [ ] High level planner.

- Tools
    - Networks; FCN, Conv, ... using Torch
    - Optimizors; CEO, Policy gradient and extensions, Deep Q learning and extensions.
    - Rewards; Distance to goal, Control values, Difference in control values, Gravity, Relaxedness
    - Formulations; All joints controlled by network, Limbs controlled by IK + co-ordinated by network.

| act/plan           | per limb ik | q constraints | multi limb co-orindation | com over holds |
| ---                | ---         | ---           | ---                      | ---            |
| 1 NR               | done        | done          | N/A                      | done           |
| 2 NR               | done        | -             | -                        | -              |
| 2 NR + core        | done        | -             | -                        | -              |
| 2 NR + core + 2 NR | done        | -             | -                        | -              |

- Tried networks
    - [x] NR: ls, qs, goal input -> delta_qs
    - [x] NR: xis, yis, goal input -> delta_qs
- [x] Implement JT + COM control for 1 NR agent.
    - [x] Discounted com control for q_i by 1 / i
    - [ ] Local maxima problem ys = 0.
    - [ ] Powerful vs balanced tuning
    - [ ] How to use constraints well? Agents get stuck due to them even for cases where there is a solution. Local planning minumum problem.
        - Maybe choose goal only in field of view. But that changes wildly due to com control.
- [ ] Understand jacobian transpose derivation properly
- [ ] Understand neural network as an extension to jacobian transpose optimization.

#### Demos
- [ ] NR goal + com
    - [ ] goal oriented (powerful)
    - [ ] com oriented (balanced)
- [ ] worm NR goal + com

### Future work
- [ ] 2D PRM/A\*
    - [ ] Avoid duplication of same start and finish
- [ ] Belay rope
- [ ] Angle bound obstacles, replanning in known environment
- [ ] Prismatic joints
- [ ] Springy limbs
- [ ] Spatial data structures
- [ ] Unknown environment (note: the milestones can't be sampled they are part of environment)
