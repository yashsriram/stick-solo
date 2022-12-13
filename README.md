# stick-solo

## description
- This work illustrates hierarchical control.
    - Limb-end level control: Using neural network as policy, which is optimized using cross-entropy method.
    - Inverse-kinematics level control: Using random sampling based near global solves + gradient descent based snapping to goal.

### Assumptions
- 2D wall and 2D constrained stick-figure agents.
- Rigid links + revolute joints.
- Links arranged serially.
- N links <=> N joints <=> N sized joint tuple <=> N + 1 ends.
- Always one pivot end and one free end.

### Pinned Chain couple agent
- Two Chain agents connected end to end.
- Always one holding end and one free end.
- The holding Chain's free end determines non-holding Chain's pivot at any instant.

## usage
- Use `cargo +nightly run --release --bin <bin crate>` to run a bin crate.
- Common controls
    - `w a s d` and `i j k l` for goal control.
    - `- +` for camera zoom in and out movements.
    - `arrow keys` camera panning.

## demonstration
### bin crates
- `gd_*` - illustrates vanilla gradient descent ik control for Chain agent as two limb agent.
- `relax_gd_transfers` - illustrates relaxing heuristic + gradient descent ik control for Chain agent as two limb agent.
- `nprs_gd_*` - illustrates no prior random sampling + gradient descent ik control for Chain agent as two limb agent.
- `csrs_gd_*` - illustrates current state random sampling + gradient descent ik control for Chain agent as two limb agent.
- `ohc_reach_manual` - illustrates (manual neck control) + (no prior random sampling + gradient descent ik control) for Pinned Chain couple agent as two limb agent.
- `ohc_reach_ceo` - illustrates (optimizing a network for neck control using cross entropy method) + (no prior random sampling + gradient descent ik control) for Pinned Chain couple agent as two limb agent.
    - Seperate networks are independently optimized left holding and right holding cases.
    - After training for a case the optimized network is written to a file.
- `ohc_transfers` - illustrates (network neck control) + (no prior random sampling + gradient descent ik control) for Pinned Chain couple agent as two limb agent.
    - It takes two command line arguments, path to neural network file for left case and right case respectively.
    - Given these two networks, depending on current case it uses appropriate network to control neck.
- `ohc_plot` and `plotting/plot3d.m` are used together for analyzing and plotting some graphs for a given optimized neural network.

## writing and demos for website

- There is an effort to emulate some physics, such as gravity effects by pushing center of mass downwards. But there is no seperation of agent and environment.
- Tried networks.
    - Chain: ls, qs, goal input -> delta_qs.
    - Chain: xis, yis, goal input -> delta_qs.
- Local com control.
    - Implement COMx control.
        - delta_q1 = 2 * x_c * dx/dq1; not = dx/dqq; i.e. min x_c^2 not x_c.
        - Discounted com control for q_i by 1 / i.
        - Sending com to origin vs origin + goal / 2. Can actually send anywhere.
        - Optimized calculation.
    - COMy control. push com_y downward.
    - Local maxima problem ys = 0. (very rare problem since other controls are generally involved.).
    - Heuristics to model powering through (adrenaline).
        - gaussian randomized end control (sometimes the weight is > 1 modelling overpower).

- Center of mass realism; duct-taping.
    - Local com control.
        - Implement COMx control.
            - delta_q1 = 2 * x_c * dx/dq1; not = dx/dqq; i.e. min x_c^2 not x_c.
            - Discounted com control for q_i by 1 / i.
            - Sending com to origin vs origin + goal / 2. Can actually send anywhere.
            - Optimized calculation.
        - COMy control. push com_y downward.
        - Local maxima problem ys = 0. (very rare problem since other controls are generally involved.).
        - Heuristics to model powering through (adrenaline).
            - gaussian randomized end control (sometimes the weight is > 1 modelling overpower).

- Arbitrarily global optimal control (Random sample solve and interpolate control).
    - From the spirit of RANSAC.
    - Given end effector goal, randomly sample q vector (in q clamps range) and keep the q\* which achieves closest approach.
    - This at limit should not be stuck at local minima. Therefore is bit different from gradient descent.
    - These iterations can be stopped after a fixed number of samples or if closest approach is less than a threshold.
    - Given q\* just interpolate from current q to q\*.
    - Parallelizable.

- Arbitrarily global optimal control (Cross-entropy solve and interpolate control).
    - From the spirit of CEO.
    - Improvement. Instead of sampling randomly in whole q clamp, sample in small region around q, take the best q\*, then sample in vicinity of q\* and so on.
    - More prone to local minima but given enough big sampling region local minima can be avoided.

- Global optimal planning (Solving local planning minumum problem. Agents get stuck due to them even for cases where there is a solution).
    - 1. Heuristics to reduce local minima.
        - relaxation time (theoretically guaranteed local minima problem solve given enough relaxation time).
    - 2. View it as a two link chain (decrease degree of freedom) (Don't want to implement now).
    - 3. Random global optimal solve.
    - 4. Cross entropy global optimal solve.

- Reaching a hold.
    - Local planners.
    - Global planners. How to snap to hold once close enough (Give responsibility to local planner).
    - Weights of both planners as a function of ticks.
    - Restrict q0 sampling.
    - Optimize sorting in genetic planners.

- Switching pivot.
    - q and q clamp assignment on switching (refer to code for math and why q1 clamp has to be (-inf, inf)).

- Matching hands. If your right hand is free and next hold is on your left; switch hands.
    - using goal_reached_slack in deciding to match hands;
        ```rust
        let have_to_match = match pivoting_side {
            Side::Left => given_goal[0] - origin[0] < -Chain::GOAL_REACHED_SLACK,
            Side::Right => given_goal[0] - origin[0] > Chain::GOAL_REACHED_SLACK,
        };
        ```
    - But now the end of the hand can be atmost 2 * GOAL_REACHED_SLACK from the hold.
    - And to visualize this we need sqaures of size 4 * GOAL_REACHED_SLACK from the hold.

- Learning based neck position predictor + (Monte-carlo + gradient descent) based inverse kinematic controller for chains.

- 2 limb as 2 switching Chain.
    - [x] Enforcing constraints - (no more constraints; uses previous constraints).
    - [x] Formulating as RL problem.
        - [x] Very nice visualization of holding goal w.r.t non-holding goal.
        - [x] Useful for debugging, reward function design and testing.
            - since the output of network does not depend on qs (initial state) if every point is roguhly tested it is enough, O(N) no need to test every pair of points (src, dest) O(N^2).
        - [x] Encoding input.
        - [x] Decoding output.
        - [x] Reward function design.
            - Explain why end of episode comy reward is a bad one (If the goal is high up and the agent reaches it; its com y will be high resulting in a lower reward for good behavior).
            - This is also a problem with in episode comy reward but (giving less weight to it and having a lot of episodes per batch asymptotically mitigates it).
        - [x] FCN design.
        - [x] CEO parameters tuning.
        - [x] Left, right holding seperate networks.
        - [x] Ensure and showcase mostly working, holding origin invariance, scale invariance.
        - [x] Auto scale goal region based on holding ls.
        - Discuss how non-trivial the deciding holding goal and how RL is a good tool here (different positions, non-trivial gaits, discontinuity at ends).
    - [x] Switcing and matching (transfers).
    - [x] Can even make a full network visualization ((x,y)_non_holding_goal vs dist(x,y)_holding_goal_from_origin).
    - [x] Left and right holding single network (since the current task can be seperated into mutually exclusive and exhaustive problems; left holding and right holding, seperate networks for each would work).
    - [x] Improve comy behavior (loss function itself and tuning).
    - [x] Improve visuals (stich sprites).
    - [x] Improve the transfers demo.
    - [x] Uneven climbing agents (ohc reach).

- [x] 2 limb as 4R: (1 x try various weights)
    - differs from Chain iterative traversing agent in baseline as mentioned by above reasons.
    - [x] Illustrate q and delta q constraints.
    - [x] reaching: local.
        - [x] Only end control (2).
        - [x] COM controls (2 x 1).
        - [x] Local minima stuck.
            - crossing hands.
            - top to bottom not on wrong side.
            - bottom to side not too much.
    - [x] reaching: global.
        - [x] Show normal scenarios.
        - [x] Solve local minima stuck, using relaxing, random solve, ceo solve.
        - [x] Also show the use case of q0 clamping.
            - crossing hands.
            - top to bottom not on wrong side.
            - bottom to side not too much.
        - [x] Compare quality of motions.
        - [x] Compare convergence errors and times of random and ceo.
        - [x] Since the q* is achieved randomly, same route generates different motions (variations).
    - [x] reaching and switching (transfer).
        - [x] Successful scenario.
        - [x] A scenario which needs matching.
    - [x] reaching, matching (if needed) and switching (transfer).
        - [x] Show as many types of transfers and possible.
- [x] 2 limb as 4R (learning): end_control + com_x_control + com_y_control + weights.
    - No real learning part.
- [x] 2 limb as Chain (worm): (1 x try various weights + 2 x 2 x 1 x try various weights for controls).

- [x] 2 limb as 2 Chain (non-learning): end_control + com_x_control + com_y_control.
- [x] 2 limb as 2 Chain (learning): end_control + com_x_control + com_y_control.
    - [x] reaching, matching (if needed) and switching (transfer).
