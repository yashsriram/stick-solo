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
- My game my rules ----------------- Physical Simulator --------------------- Real world
                        |
                current project

- Idea for human like stick figure agent.
    - [ ] IK for each limb.
    - [ ] Co-ordinate among limbs.
    - [ ] With q contrains (including one side leg goal only, no cross overs).
    - [ ] Put center of mass over holds.
    - [ ] Formulate and achieve relaxing poses/efficient transfers.
    - [ ] Do what all climb cycle agent can and more.
    - [ ] High level planner.

- Tools
    - Networks; FCN, Conv, ... using Torch
    - Optimizors; CEO, Policy gradient and extensions, Deep Q learning and extensions.
    - Rewards; Distance to goal, Time to goal, Control values, Difference in control values, Gravity, Relaxedness
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

- [x] Switching NR agent (2 limb as NR)
    - [x] q clamps
    - [x] delta q abs clamp
    - [x] Local optimal JT control
    - [ ] Local optimal pseudo inverse control
    - [x] Local com control
        - [x] Implement COMx control.
            - [x] delta_q1 = 2 * x_c * dx/dq1; not = dx/dqq; i.e. min x_c^2 not x_c
            - [x] Discounted com control for q_i by 1 / i
            - [x] Sending com to origin vs origin + goal / 2. Can actually send anywhere.
            - [x] Optimized calculation
        - [x] COMy control. push com_y downward
            - [x] Optimized calculation
        - [x] Local maxima problem ys = 0. (very rare problem since other controls are generally involved.)
        - [x] Heuristics to model powering through (adrenaline)
            - [x] gaussian randomized end control (sometimes the weight is > 1 modelling overpower)
            - [ ] Smoothen this to produce periodic spurs of energy (maybe perlin noise)
        - [ ] Powerful vs balanced tuning
    - [x] Arbitrarily global optimal control (Random sample solve and interpolate control)
        - From the spirit of RANSAC.
        - Given end effector goal, randomly sample q vector (in q clamps range) and keep the q\* which achieves closest approach.
        - This at limit should not be stuck at local minima. Therefore is bit different from gradient descent.
        - These iterations can be stopped after a fixed number of samples or if closest approach is less than a threshold.
        - Given q\* just interpolate from current q to q\*
        - Parallelizable
    - [x] Arbitrarily global optimal control (Cross-entropy solve and interpolate control)
        - From the spirit of CEO.
        - Improvement. Instead of sampling randomly in whole q clamp, sample in small region around q, take the best q\*, then sample in vicinity of q\* and so on.
        - More prone to local minima but given enough big sampling region local minima can be avoided.
    - [x] Global optimal planning (Solving local planning minumum problem. Agents get stuck due to them even for cases where there is a solution).
        - [x] 1. Heuristics to reduce local minima.
            - relaxation time (theoretically guaranteed local minima problem solve given enough relaxation time)
        - [x] 2. View it as a two link chain (decrease degree of freedom) (Don't want to implement now)
        - [x] 3. Random global optimal solve
        - [x] 4. Cross entropy global optimal solve
    - [x] Reaching a hold.
        - [x] Local planners
        - [x] Global planners. How to snap to hold once close enough (Give responsibility to local planner)
        - [x] Weights of both planners as a function of ticks.
        - [x] Restrict q0 sampling
        - [x] Optimize sorting in genetic planners
    - [x] Switching pivot.
        - [x] q and q clamp assignment on switching (refer to code for math and why q1 clamp has to be (-inf, inf))
    - [x] Matching hands. If your right hand is free and next hold is on your left; switch hands
        - [x] using goal_reached_slack in deciding to match hands;
            ```rust
            let have_to_match = match pivoting_side {
                PivotingSide::Left => given_goal[0] - origin[0] < -SwitchableNR::GOAL_REACHED_SLACK,
                PivotingSide::Right => given_goal[0] - origin[0] > SwitchableNR::GOAL_REACHED_SLACK,
            };
            ```
    - [x] Traversing a path
        - local
        - global
- [ ] 2 limb as 2 switching NRs
    - [x] Enforcing constraints - (no more constraints; uses previous constraints)
    - [ ] Formulating as RL problem
        - [x] Very nice visualization of holding goal w.r.t non-holding goal
        - [x] Useful for debugging, reward function design and testing
            - since the output of network does not depend on qs (initial state) if every point is roguhly tested it is enough, O(N) no need to test every pair of points (src, dest) O(N^2)
        - [x] Encoding input
        - [x] Decoding output
        - [x] Reward function design
            - Explain why end of episode comy reward is a bad one (If the goal is high up and the agent reaches it; its com y will be high resulting in a lower reward for good behavior)
            - This is also a problem with in episode comy reward but (giving less weight to it and having a lot of episodes per batch asymptotically mitigates it)
        - [x] FCN design
        - [x] CEO parameters tuning
        - [x] Left, right holding seperate networks
        - [x] Ensure and showcase mostly working, holding origin invariance, scale invariance
        - [x] Auto scale goal region based on holding ls
        - [ ] Left and right holding single network
        - [ ] Can even make a full network visualization ((x,y)_non_holding_goal vs dist(x,y)_holding_goal_from_origin)
        - [ ] Improve comy behavior
        - Discuss how non-trivial the deciding holding goal and how RL is a good tool here (different positions, non-trivial gaits, discontinuity at ends).
    - [ ] Switcing and matching (transfers)
- [ ] Understand jacobian transpose derivation properly
- [ ] Understand neural network as an extension to jacobian transpose optimization.

#### Demos
- Possible variants
    - (2) global optimal control or (2) local optimal controls [jt end_control, pseudo inv end_control]
    - (2) com_x controls [origin com_x_control, midpoint com_x_control]
    - (1) com_y control

- [ ] 2 limb as 4R: (1 x try various weights)
    - differs from NR iterative traversing agent in baseline as mentioned by above reasons
    - [ ] Illustrate q and delta q constraints
    - [ ] reaching: local
        - [ ] Only end control (2)
        - [ ] COM controls (2 x 1)
        - [ ] Various values
        - [ ] Local minima stuck
            - crossing hands
            - top to bottom not on wrong side
            - bottom to side not too much
    - [ ] reaching: global
        - [ ] Show normal scenarios
        - [ ] Solve local minima stuck, using relaxing, random solve, ceo solve
        - [ ] Also show the use case of q0 clamping
            - crossing hands
            - top to bottom not on wrong side
            - bottom to side not too much
        - [ ] Compare quality of motions
        - [ ] Compare convergence errors and times of random and ceo
        - [ ] Is the parallelization helpful?
        - [ ] Since the q* is achieved randomly, same route generates different motions (variations)
    - [ ] reaching and switching (transfer)
        - [ ] Successful scenario
        - [ ] A scenario which needs matching
    - [ ] reaching, matching (if needed) and switching (transfer)
        - [ ] Show as many types of transfers and possible
- [x] 2 limb as 4R (learning): end_control + com_x_control + com_y_control + weights
    - No real learning part
- [ ] 2 limb as NR (worm): (1 x try various weights + 2 x 2 x 1 x try various weights for controls)
    - [ ] Show as many types of transfers and possible

- [ ] 2 limb as 2 NR (non-learning): end_control + com_x_control + com_y_control
- [ ] 2 limb as 2 NR (learning): end_control + com_x_control + com_y_control
    - [ ] Illustrate q and delta q constraints
    - [ ] reaching: local
        - [ ] Local minima stuck
            - crossing hands
            - top to bottom not on wrong side
            - bottom to side not too much
    - [ ] reaching: global
        - [ ] Show normal scenarios
        - [ ] Solve local minima stuck, using relaxing, random solve, ceo solve
        - [ ] Since the q* is achieved randomly, same route generates different motions (variations)
    - [ ] reaching and switching (transfer)
        - [ ] Successful scenario
        - [ ] A scenario which needs matching
    - [ ] reaching, matching (if needed) and switching (transfer)
        - [ ] Show as many types of transfers and possible

### Future work
- [ ] 2 limb as 2 NR (non-learning, two simultaneous pivots)
- [ ] 4 limb as 4 NR (general, multiple simultaneous pivots)
- [ ] 2D PRM/A\*
    - [ ] Avoid duplication of same start and finish
- [ ] Belay rope
- [ ] Angle bound obstacles, replanning in known environment
- [ ] Prismatic joints
- [ ] Springy limbs
- [ ] Spatial data structures
- [ ] Unknown environment (note: the milestones can't be sampled they are part of environment)
