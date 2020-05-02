package robot.planning.prm;

import math.Vec;
import processing.core.PApplet;

import java.util.*;

public class PRM {
    public static boolean DRAW_MILESTONES = true;
    public static boolean DRAW_EDGES = false;

    final PApplet applet;
    final List<Milestone> milestones = new ArrayList<>();
    int numEdges = 0;

    public PRM(PApplet applet) {
        this.applet = applet;
    }

    private Milestone addMilestone(float x, float y, float maxEdgeLen) {
        // Generate milestone
        int newId = milestones.size();
        Milestone newMilestone = new Milestone(applet, newId, x, y);
        // Connect to its neighbours
        for (Milestone oldMilestone : milestones) {
            // If nearby then link
            if (newMilestone.position.minus(oldMilestone.position).norm() <= maxEdgeLen) {
                newMilestone.neighbours.add(oldMilestone);
                oldMilestone.neighbours.add(newMilestone);
                numEdges++;
            }
        }
        // Add to existing
        milestones.add(newMilestone);
        return newMilestone;
    }

    public int grow(int num, Vec minCorner, Vec maxCorner, float maxEdgeLen) {
        for (int i = 0; i < num; ++i) {
            addMilestone(
                    applet.random(minCorner.get(0), maxCorner.get(0)),
                    applet.random(minCorner.get(1), maxCorner.get(1)),
                    maxEdgeLen
            );
        }
        return numEdges;
    }

    public void draw() {
        for (Milestone milestone : milestones) {
            milestone.draw();
        }
    }

    private void resetSearchState(final Vec finishPosition) {
        PApplet.println("Resetting search states of vertices");
        for (Milestone v : milestones) {
            v.searchState.reset(finishPosition);
        }
    }

    private void addToFringe(final Stack<Milestone> fringe, final Milestone current, final Milestone next) {
        fringe.add(next);
        next.searchState.addToFringeFrom(current);
    }

    public List<Vec> dfs(final Vec startPosition, final Vec finishPosition, float maxEdgeLen) {
        PApplet.println("DFS");

        Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), maxEdgeLen);
        Milestone finish = addMilestone(finishPosition.get(0), finishPosition.get(1), maxEdgeLen);
        resetSearchState(finishPosition);

        int numVerticesExplored = 0;
        final Stack<Milestone> fringe = new Stack<>();
        addToFringe(fringe, start, start);
        while (fringe.size() > 0) {
            // Pop one vertex
            Milestone current = fringe.pop();
            numVerticesExplored++;
            // Check if finishMilestone
            if (current.id == finish.id) {
                PApplet.println("Reached finish, # vertices explored: " + numVerticesExplored);
                return finish.searchState.pathFromStart;
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

        PApplet.println("Could not reach finishMilestone, # vertices explored: " + numVerticesExplored);
        return Collections.singletonList(start.position);
    }

    private void addToFringe(final Queue<Milestone> fringe, final Milestone current, final Milestone next) {
        next.searchState.distanceFromStart = current.searchState.distanceFromStart + next.position.minus(current.position).norm();
        fringe.add(next);
        next.searchState.addToFringeFrom(current);
    }

    private List<Vec> search(final Queue<Milestone> fringe, final Vec startPosition, final Vec finishPosition, float maxEdgeLen) {
        Milestone start = addMilestone(startPosition.get(0), startPosition.get(1), maxEdgeLen);
        Milestone finish = addMilestone(finishPosition.get(0), finishPosition.get(1), maxEdgeLen);
        resetSearchState(finishPosition);

        int numVerticesExplored = 0;
        addToFringe(fringe, start, start);
        while (fringe.size() > 0) {
            // Pop one vertex
            Milestone current = fringe.remove();
            numVerticesExplored++;
            // Check if finish
            if (current.id == finish.id) {
                PApplet.println("Reached finish, # vertices explored: " + numVerticesExplored);
                return finish.searchState.pathFromStart;
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

        PApplet.println("Could not reach finish, # vertices explored: " + numVerticesExplored);
        return Collections.singletonList(start.position);
    }

    public List<Vec> bfs(Vec startPosition, Vec finishPosition, float maxEdgeLen) {
        PApplet.println("BFS");
        return search(new LinkedList<>(), startPosition, finishPosition, maxEdgeLen);
    }

    public List<Vec> ucs(Vec startPosition, Vec finishPosition, float maxEdgeLen) {
        PApplet.println("UCS");
        return search(new PriorityQueue<>((v1, v2) ->
                        (int) (v1.searchState.distanceFromStart - v2.searchState.distanceFromStart)),
                startPosition, finishPosition, maxEdgeLen);
    }

    public List<Vec> aStar(Vec startPosition, Vec finishPosition, float maxEdgeLen) {
        PApplet.println("A*");
        return search(new PriorityQueue<>((v1, v2) -> (int) (
                        (v1.searchState.distanceFromStart + v1.searchState.heuristicDistanceToFinish)
                                - (v2.searchState.distanceFromStart + v2.searchState.heuristicDistanceToFinish))),
                startPosition, finishPosition, maxEdgeLen);
    }

    public List<Vec> weightedAStar(Vec startPosition, Vec finishPosition, float maxEdgeLen, final float epislon) {
        PApplet.println("Weighted A* with epsilon = " + epislon);
        return search(new PriorityQueue<>((v1, v2) -> (int) (
                        (v1.searchState.distanceFromStart + epislon * v1.searchState.heuristicDistanceToFinish)
                                - (v2.searchState.distanceFromStart + epislon * v2.searchState.heuristicDistanceToFinish))),
                startPosition, finishPosition, maxEdgeLen);
    }
}
