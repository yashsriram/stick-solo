package robot.planning.prm;

import math.Vec;
import processing.core.PApplet;
import robot.sensing.PositionConfigurationSpace;

import java.util.*;

public class PRM {
    public static boolean DRAW_MILESTONES = false;
    public static boolean DRAW_EDGES = false;
    public float margin = 0;

    private final PApplet applet;
    private final List<Milestone> milestones = new ArrayList<>();
    private int numEdges = 0;

    public PRM(PApplet applet) {
        this.applet = applet;
    }

    private Milestone addMilestone(float x, float y, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        // Generate milestone
        int newId = milestones.size();
        Milestone newMilestone = new Milestone(applet, newId, x, y);
        // Connect to its neighbours
        for (Milestone oldMilestone : milestones) {
            // If not in min max range do not connect
            float distance = newMilestone.position.minus(oldMilestone.position).norm();
            if (distance < minEdgeLen || distance > maxEdgeLen) {
                continue;
            }
            // If intersects an obstacle do not connect
            if (cs.doesIntersect(newMilestone.position, oldMilestone.position)) {
                continue;
            }
            // If all ok connect
            newMilestone.neighbours.add(oldMilestone);
            oldMilestone.neighbours.add(newMilestone);
            numEdges++;
        }
        // Add to existing
        milestones.add(newMilestone);
        return newMilestone;
    }

    public int grow(int num, Vec minCorner, Vec maxCorner, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        for (int i = 0; i < num; ++i) {
        	float milestoneX = applet.random(minCorner.get(0), maxCorner.get(0));
        	float milestoneY = applet.random(minCorner.get(1), maxCorner.get(1));
        	while(cs.doesIntersect(new Vec(milestoneX, milestoneY), this.margin)) {
        		milestoneX = applet.random(minCorner.get(0), maxCorner.get(0));
            	milestoneY = applet.random(minCorner.get(1), maxCorner.get(1));
        	}
            addMilestone(
                    milestoneX,
                    milestoneY,
                    minEdgeLen,
                    maxEdgeLen,
                    cs
            );
        }
        return numEdges;
    }

    public void draw() {
        for (Milestone milestone : milestones) {
            milestone.draw();
        }
    }

    private void resetSearchState(final Vec goalPosition) {
        PApplet.println("Resetting search states of vertices");
        for (Milestone v : milestones) {
            v.searchState.reset(goalPosition);
        }
    }

    private void addToFringe(final Stack<Milestone> fringe, final Milestone current, final Milestone next) {
        fringe.add(next);
        next.searchState.addToFringeFrom(current);
    }

    public List<Milestone> dfs(final Vec startPosition, final Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        PApplet.println("DFS");

        Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), minEdgeLen, maxEdgeLen, cs);
        Milestone goal = addMilestone(goalPosition.get(0), goalPosition.get(1), minEdgeLen, maxEdgeLen, cs);
        resetSearchState(goalPosition);

        int numVerticesExplored = 0;
        final Stack<Milestone> fringe = new Stack<>();
        addToFringe(fringe, start, start);
        while (fringe.size() > 0) {
            // Pop one vertex
            Milestone current = fringe.pop();
            numVerticesExplored++;
            // Check if goal milestone
            if (current.id == goal.id) {
                PApplet.println("Reached goal, # vertices explored: " + numVerticesExplored);
                return goal.searchState.pathFromStart;
            }
            // Mark this vertex as explored
            current.searchState.setExplored();
            // Update fringe
            for (Milestone neighbour : current.neighbours) {
                if (!neighbour.searchState.isExplored) {
                    addToFringe(fringe, current, neighbour);
                }
            }
        }

        PApplet.println("Could not reach goal milestone, # vertices explored: " + numVerticesExplored);
        return Collections.singletonList(start);
    }

    private void addToFringe(final Queue<Milestone> fringe, final Milestone current, final Milestone next) {
        next.searchState.distanceFromStart = current.searchState.distanceFromStart + next.position.minus(current.position).norm();
        fringe.add(next);
        next.searchState.addToFringeFrom(current);
    }

    private List<Milestone> search(final Queue<Milestone> fringe, final Vec startPosition, final Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), minEdgeLen, maxEdgeLen, cs);
        Milestone goal = addMilestone(goalPosition.get(0), goalPosition.get(1), minEdgeLen, maxEdgeLen, cs);
        resetSearchState(goalPosition);

        int numVerticesExplored = 0;
        addToFringe(fringe, start, start);
        while (fringe.size() > 0) {
            // Pop one vertex
            Milestone current = fringe.remove();
            numVerticesExplored++;
            // Check if goal
            if (current.id == goal.id) {
                PApplet.println("Reached goal, # vertices explored: " + numVerticesExplored);
                return goal.searchState.pathFromStart;
            }
            // Mark this vertex as explored
            current.searchState.setExplored();
            // Update fringe
            for (Milestone neighbour : current.neighbours) {
                if (!neighbour.searchState.isExplored) {
                    addToFringe(fringe, current, neighbour);
                }
            }
        }

        PApplet.println("Could not reach goal, # vertices explored: " + numVerticesExplored);
        return Collections.singletonList(start);
    }

    public List<Milestone> bfs(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        PApplet.println("BFS");
        return search(new LinkedList<>(), startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
    }

    public List<Milestone> ucs(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        PApplet.println("UCS");
        return search(new PriorityQueue<>((v1, v2) ->
                        (int) (v1.searchState.distanceFromStart - v2.searchState.distanceFromStart)),
                startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
    }

    public List<Milestone> aStar(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs) {
        PApplet.println("A*");
        return search(new PriorityQueue<>((v1, v2) -> (int) (
                        (v1.searchState.distanceFromStart + v1.searchState.heuristicDistanceToGoal)
                                - (v2.searchState.distanceFromStart + v2.searchState.heuristicDistanceToGoal))),
                startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
    }

    public List<Milestone> weightedAStar(Vec startPosition, Vec goalPosition, float minEdgeLen, float maxEdgeLen, PositionConfigurationSpace cs, final float epislon) {
        PApplet.println("Weighted A* with epsilon = " + epislon);
        return search(new PriorityQueue<>((v1, v2) -> (int) (
                        (v1.searchState.distanceFromStart + epislon * v1.searchState.heuristicDistanceToGoal)
                                - (v2.searchState.distanceFromStart + epislon * v2.searchState.heuristicDistanceToGoal))),
                startPosition, goalPosition, minEdgeLen, maxEdgeLen, cs);
    }
    
	public void removeMilestones(List<Milestone> milestones) {
		for(Milestone milestone: milestones) {
			for(Milestone neighbor: milestone.neighbours) {
	    		neighbor.neighbours.remove(milestone);
	    	}
	    	this.milestones.remove(milestone);
		}
		
	}
}
