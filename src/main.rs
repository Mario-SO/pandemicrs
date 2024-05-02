use rand::{thread_rng, Rng};
use raylib::prelude::*;
use std::time::{Duration, Instant};

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const FPS: u32 = 60;
const HUMAN_COUNT: i32 = 1000;
const INFECTION_RANGE: i32 = 10;
const INFECTION_TIME: Duration = Duration::new(3, 0); // Time required to infect

struct Human {
    pos_x: i32,
    pos_y: i32,
    is_infected: bool,
    infection_start: Option<Instant>,
}

impl Human {
    fn new(x: i32, y: i32) -> Self {
        Human {
            pos_x: x,
            pos_y: y,
            is_infected: false,
            infection_start: None,
        }
    }

    fn update_position(&mut self) {
        let mut rng = thread_rng();
        self.pos_x += rng.gen_range(-1..=1);
        self.pos_y += rng.gen_range(-1..=1);
        // Ensure humans stay within window bounds
        self.pos_x = self.pos_x.clamp(0, WINDOW_WIDTH - 10);
        self.pos_y = self.pos_y.clamp(0, WINDOW_HEIGHT - 10);
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let color = if self.is_infected {
            Color::RED
        } else {
            Color::WHITE
        };
        d.draw_pixel(self.pos_x, self.pos_y, color);
    }
    // implement check proximity

    fn check_proximity(&self, other: &Human, range: i32) -> bool {
        let dx = self.pos_x - other.pos_x;
        let dy = self.pos_y - other.pos_y;
        let distance_squared = dx * dx + dy * dy;
        distance_squared <= range * range
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Pandemic Simulation")
        .build();
    rl.set_target_fps(FPS);

    let mut humans: Vec<Human> = (0..HUMAN_COUNT)
        .map(|_| {
            Human::new(
                thread_rng().gen_range(0..WINDOW_WIDTH),
                thread_rng().gen_range(0..WINDOW_HEIGHT),
            )
        })
        .collect();

    // Infect a few humans initially
    for human in humans.iter_mut().take(5) {
        human.is_infected = true;
    }
    let mut infected = 5;
    let mut healthy: i32 = HUMAN_COUNT - 5;

    while !rl.window_should_close() {
        let current_time = Instant::now();
        let fps = rl.get_fps();

        // Update positions independently from infection checking
        for human in &mut humans {
            human.update_position();
        }

        // Checking for infections
        for i in 0..humans.len() {
            // We don't update infection status within this same loop to avoid conflicts
            let mut close_contacts = 0; // Reset for each human checked
            for j in 0..humans.len() {
                if i != j && humans[i].check_proximity(&humans[j], INFECTION_RANGE) {
                    close_contacts += 1;
                }
            }

            if close_contacts > 0 {
                let human = &mut humans[i];
                match human.infection_start {
                    None => human.infection_start = Some(current_time),
                    Some(start) => {
                        if current_time.duration_since(start) >= INFECTION_TIME {
                            if !human.is_infected {
                                human.is_infected = true;
                                human.infection_start = None;
                                infected += 1;
                                healthy -= 1;
                            }
                        }
                    }
                }
            }
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        for human in &humans {
            human.draw(&mut d);
        }
        d.draw_text(
            &format!("FPS: {}", fps.to_string().as_str()),
            10,
            10,
            13,
            Color::RAYWHITE,
        );
        d.draw_text(
            &format!("Infected: {}", infected.to_string().as_str()),
            10,
            35,
            13,
            Color::RED,
        );
        d.draw_text(
            &format!("Healthy: {}", healthy.to_string().as_str()),
            10,
            55,
            13,
            Color::GREEN,
        );
    }
}
