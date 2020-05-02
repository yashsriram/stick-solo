package robot.planning.prm;

import math.Vec;
import processing.core.PApplet;

import java.util.ArrayList;
import java.util.List;

class Milestone {
    final PApplet applet;
    final int id;
    final Vec position;
    final List<Milestone> neighbours = new ArrayList<>();

    class SearchState {
        float distanceFromStart = 0;
        float heuristicDistanceToGoal = 0;
        boolean isExplored = false;
        final List<Vec> pathFromStart = new ArrayList<>();
        final Vec color;

        public SearchState() {
            this.color = new Vec(1, 1, 1);
        }

        void reset(Vec goalPosition) {
            distanceFromStart = 0;
            heuristicDistanceToGoal = position.minus(goalPosition).norm();
            isExplored = false;
            pathFromStart.clear();
            color.headSet(1, 1, 1);
        }

        void addToFringeFrom(Milestone parent) {
            isExplored = true;
            pathFromStart.addAll(parent.searchState.pathFromStart);
            pathFromStart.add(Milestone.this.position);
            color.headSet(0, 1, 0);
        }

        void setExplored() {
            color.headSet(1, 0, 0);
        }
    }

    final SearchState searchState;

    Milestone(PApplet applet, int id, float x, float y) {
        this.applet = applet;
        this.id = id;
        this.position = new Vec(x, y);
        this.searchState = new SearchState();
    }

    void draw() {
        if (PRM.DRAW_MILESTONES) {
            // Milestone
            applet.stroke(searchState.color.get(0), searchState.color.get(1), searchState.color.get(2));
            applet.point(0, position.get(1), position.get(0));
        }
        if (PRM.DRAW_EDGES) {
            // Edges
            for (Milestone neighbour : neighbours) {
                applet.stroke(1);
                applet.line(0, position.get(1), position.get(0),
                        0, neighbour.position.get(1), neighbour.position.get(0));
            }
        }
    }

}
