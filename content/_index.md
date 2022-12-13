+++
+++

 This project models climbing agents using arm manipulators. It illustrates the inverse kinematics of manipulators using closed-form, open-form, sample-based methods. It incorporates center of mass term in inverse kinematics to generate a natural looking motion. It illustrates the need and versetality of using a learning based method for a particular situation. It shows a deep RL method to predict optimal neck position for hold to hold movement.


- The tour of 1. Link to youtube video. Click to play.

[![](http://img.youtube.com/vi/bZg6pS2gGPw/0.jpg)](https://www.youtube.com/watch?v=bZg6pS2gGPw)

- The final result of 2.

![](./github/12.gif)

| Gradient descent IK control | Neural network IK control |
| --- | --- |
| ![](./github/0.1.ik.gif) | ![](./github/0.2.ceo.gif) |
| Has smooth control | Has noisy control and difficult to train generally |

Therefore direct neural-network IK control is not used.

- Baseline.

![](./github/1.jt.gif)

![](./github/2.cc.gif)

- New system intro.

![](./github/3.0.intro.gif)

- Effect of center of mass term in gradient descent IK.

![](./github/3.1.com.gif)

| Local minima problem | Its solution |
| --- | --- |
| ![](./github/4.0.local_minima.gif) | ![](./github/4.1.solve_local_minima.gif) |

| Cartwheeling problem | Its solution |
| --- | --- |
| ![](./github/5.0.wrong_side.gif) | ![](./github/5.1.solve_wrong_side.gif) |

- Due to inherent usage of randomness, different motion is produced for same scenarios.

![](./github/6.4.out.gif)

![](./github/6.jpg)

- Switching pivot to continue on a path.

![](./github/7.1.switching_success.gif)

- Failure case of switching pivot.

![](./github/7.2.switching_failure.gif)

- Matching hands (when needed) + switching pivot solves the problem.

![](./github/7.3.matching.gif)

- Chain as two limb agent on a climbing route: Vanilla gradient-descent control.

![](./github/8.1.gd.gif)

- Chain as two limb agent on a climbing route: Relax on every hold + gradient-descent control.

![](./github/8.2.relax.gif)

- Chain as two limb agent on a climbing route: No-prior random-sample near-global solve + gradient-descent snapping control.

![](./github/8.3.nprs.gif)

- Chain as two limb agent on a climbing route: Current-state random-sample near-global solve + gradient-descent snapping control.

![](./github/8.4.csrs.gif)

- Large N with small lengths can model a worm.

![](./github/9.worm.gif)

- Pinned Chain couple as two limb agent.
    - Manual neck and reaching hand goal control.
    - No-prior random-sample near-global solve + gradient-descent snapping IK control.

![](./github/10.ohc_reach.gif)

- Pinned Chain couple as two limb agent.
    - Holding with left arm.
    - Manual reaching hand goal control.
    - Cross-entropy optimized network neck control.
    - No-prior random-sample near-global solve + gradient-descent snapping IK control.

![](./github/11.1.gif)

- Pinned Chain couple as two limb agent.
    - Holding with right arm.
    - Manual reaching hand goal control.
    - Cross-entropy optimized network neck control.
    - No-prior random-sample near-global solve + gradient-descent snapping IK control.

![](./github/11.2.gif)

- Visualizing neural network policy for neck using texture map distortion method.
    - Input to network is lengths of agent links and reaching hand goal.
    - Output is neck goal.
    - Since for a given agent lengths are fixed, policy is a map from R^2 -> R^2.
    - Left map shows original texture with linear mapping y = x.
    - Right map shows texture distorted using policy y = f(x).

![](./github/11.3.gif)

- Cross-entropy optimization can be parallelized. Time taken for optimization is shown for number of CPU cores.

![](./github/parallelization.png)

- Pinned Chain couple as two limb agent.
    - Manual reaching hand goal control (derived from given path).
    - Cross-entropy optimized network neck control.
    - No-prior random-sample near-global solve + gradient-descent snapping IK control.

![](./github/12.gif)

