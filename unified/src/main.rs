use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, timer};
use ggez::{graphics::DrawParam, graphics::Rect, Context, GameResult};
use std::env;
use std::path;

use nalgebra::Point2;
use stick_solo::plan::prm::PRM;
use stick_solo::sense::{
    obstacle::{CircleObstacle, LineSegmentObstacle},
    pos_conf_space::PosConfSpace,
};
use stick_solo::Draw;

struct MainState {
    pos_conf_space: PosConfSpace,
    prm: PRM,
}

impl MainState {
    const DESIRED_FPS: u32 = 60;
    fn new() -> GameResult<MainState> {
        let pos_conf_space = PosConfSpace {
            obstacles: vec![
                Box::new(CircleObstacle::new(Point2::new(400.0, 400.0), 100.0)),
                Box::new(CircleObstacle::new(Point2::new(200.0, 100.0), 100.0)),
                Box::new(LineSegmentObstacle::new(
                    Point2::new(000.0, 000.0),
                    Point2::new(200.0, 100.0),
                )),
            ],
        };
        use std::time::Instant;
        let now = Instant::now();
        let prm = PRM::new(
            Rect::new(50.0, 100.0, 1000.0, 900.0),
            1000,
            50.0,
            &pos_conf_space,
            1.0,
        );
        println!("{} ms", now.elapsed().as_millis());
        Ok(MainState {
            pos_conf_space,
            prm,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, MainState::DESIRED_FPS) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        self.pos_conf_space.draw(ctx)?;
        self.prm.draw(ctx)?;
        graphics::present(ctx)?;
        graphics::set_window_title(ctx, &format!("FPS: {:.00}", ggez::timer::fps(ctx)));

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let result = match keycode {
            KeyCode::W => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().dest([0.0, -20.0]),
                ),
            ),
            KeyCode::A => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().dest([-20.0, 0.0]),
                ),
            ),
            KeyCode::S => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().dest([0.0, 20.0]),
                ),
            ),
            KeyCode::D => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().dest([20.0, 0.0]),
                ),
            ),
            KeyCode::Q => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().scale([1.1, 1.1]),
                ),
            ),
            KeyCode::E => graphics::set_screen_coordinates(
                ctx,
                graphics::transform_rect(
                    graphics::screen_coordinates(ctx),
                    DrawParam::new().scale([0.9, 0.9]),
                ),
            ),
            _ => Ok(()),
        };
        if let Err(msg) = result {
            println!("{:?}", msg);
        }
        if keycode == KeyCode::Escape {
            ggez::event::quit(ctx);
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = cb.build()?;

    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}

// public class CircularAgentOnPRM extends PApplet {
//     private static final int SIZE = 100;
//     private static String SEARCH_ALGORITHM = "";

//     private static final Vec MIN_CORNER = new Vec(-SIZE, -SIZE);
//     private static final Vec MAX_CORNER = new Vec(SIZE, SIZE);
//     private static final Vec START_POSITION = new Vec(SIZE * (-9f / 10), SIZE * (9f / 10));
//     private static final Vec GOAL_POSITION = new Vec(SIZE * (9f / 10), SIZE * (-9f / 10));
//     private static final float MAX_EDGE_LEN = 7;

//     CircularAgent circularAgent;

//     public void setup() {
//         surface.setTitle("Processing");
//         colorMode(RGB, 1.0f);
//         rectMode(CENTER);

//         cam = new QueasyCam(this);
//         cs = new PositionConfigurationSpace(this, List.of());
//         prm = new PRM(this);
//         int numMilestones = 2000;
//         int numEdges = prm.grow(numMilestones, MIN_CORNER, MAX_CORNER, 0, MAX_EDGE_LEN, cs);
//         PApplet.println("# milestones : " + numMilestones + " # edges : " + numEdges);
//         circularAgent = new CircularAgent(this, START_POSITION, 3, 10, new Vec(1, 1, 1));
//     }

//     public void draw() {
//         if (keyPressed) {
//             if (keyCode == RIGHT) {
//                 circularAgent.stepForward();
//             }
//             if (keyCode == LEFT) {
//                 circularAgent.stepBackward();
//             }
//         }
//         long start = millis();
//         // update
//         circularAgent.update(0.1f);
//         long update = millis();
//         // draw
//         background(0);
//         // agent
//         circularAgent.draw();
//         // graph
//         prm.draw();
//         long draw = millis();

//         surface.setTitle("Processing - FPS: " + Math.round(frameRate) + " Update: " + (update - start) + "ms Draw " + (draw - update) + "ms" + " search: " + SEARCH_ALGORITHM);
//     }

//     public void keyPressed() {
//         if (key == 'p') {
//             circularAgent.isPaused = !circularAgent.isPaused;
//         }
//         if (key == 'k') {
//             PRM.DRAW_MILESTONES = !PRM.DRAW_MILESTONES;
//         }
//         if (key == 'j') {
//             PRM.DRAW_EDGES = !PRM.DRAW_EDGES;
//         }
//         if (key == '1') {
//             circularAgent.spawn(START_POSITION, prm.dfs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, cs));
//             SEARCH_ALGORITHM = "DFS";
//         }
//         if (key == '2') {
//             circularAgent.spawn(START_POSITION, prm.bfs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, cs));
//             SEARCH_ALGORITHM = "BFS";
//         }
//         if (key == '3') {
//             circularAgent.spawn(START_POSITION, prm.ucs(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, cs));
//             SEARCH_ALGORITHM = "UCS";
//         }
//         if (key == '4') {
//             circularAgent.spawn(START_POSITION, prm.aStar(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, cs));
//             SEARCH_ALGORITHM = "A*";
//         }
//         if (key == '5') {
//             circularAgent.spawn(START_POSITION, prm.weightedAStar(START_POSITION, GOAL_POSITION, 0, MAX_EDGE_LEN, cs, 1.5f));
//             SEARCH_ALGORITHM = "weighted A*";
//         }
//     }
