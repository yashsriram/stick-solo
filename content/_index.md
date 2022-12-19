+++
+++

<img src="github/12.gif" width=100%>

__Motion planning for wall climbing__ is discussed in this project.
In wall climbing, an agent starts with an initial pose and then uses protrusions on the wall (called holds) to lock onto and climbs to the finish hold at the top.
In this project, the goal is for the agent to reach the __finish__ hold moving as __naturally__ as possible.
We illustrate that __joint angles constraints__ and __center of mass control__ contribute significantly to natural motion synthesis.
Local minima poses are avoided by using __random sampling based asymptotically globally-optimal inverse-kinematics solves__.
These coupled with gradient descent make the agent reach snap to holds reliably.
The classical climbing techniques of _switching pivots_ and _matching hands_ are also programmed.
The __neck position prediction__ for a two arm agent is non-trivial and a good candidate for learning based methods.
We propose a __neural network__ based __policy__ trained using __cross-entropy optimizer__ for this task.
We visualize and provide insights into the learnt policy.
Using these methods we are able to send the routes reliably with natural looking motion.

