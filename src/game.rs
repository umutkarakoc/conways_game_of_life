use std::collections::HashMap;

use bevy::{
    input::{gamepad::GamepadAxisChangedEvent, keyboard::KeyboardInput},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Pause,
    Play,
}

#[derive(SystemSet, Debug, Eq, PartialEq, Clone, Hash)]
pub struct Calculate;

#[derive(Resource, Deref, DerefMut)]
pub struct Zoom(f32);

#[derive(Resource)]
pub struct Cells(HashMap<(i32, i32), bool>);

#[derive(Resource)]
pub struct CellEntities(HashMap<(i32, i32), Entity>);

#[derive(Component)]
pub struct Cell {
    x: i32,
    y: i32,
}

#[derive(Event)]
pub struct NextStepEvent;

#[derive(Resource)]
pub struct GameTimer(Timer);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let mut cells = HashMap::new();

        for x in -100..100 {
            for y in -100..100 {
                cells.insert((x, y), false);
            }
        }

        app.insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
            .insert_resource(Zoom(0.1))
            .insert_resource(Cells(cells))
            .insert_resource(CellEntities(HashMap::new()))
            .insert_resource(GameTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
            .add_event::<NextStepEvent>()
            .add_state::<GameState>()
            .add_systems(Startup, init)
            .add_systems(Update, run_timer.run_if(in_state(GameState::Play)))
            .add_systems(
                Update,
                (pause_play, manuel_step, step, create).in_set(Calculate),
            )
            .add_systems(Update, render.after(Calculate));
    }
}

fn init(
    mut commands: Commands,
    res: Res<Zoom>,
    cells: Res<Cells>,
    mut entities: ResMut<CellEntities>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.),
        projection: OrthographicProjection {
            scale: res.0,
            ..default()
        },
        ..default()
    });

    for ((x, y), _) in &cells.0 {
        let x = *x;
        let y = *y;
        let entity = commands
            .spawn((
                Cell { x, y },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0., 0., 0.),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.)),
                    visibility: Visibility::Hidden,
                    ..default()
                },
            ))
            .id();
        entities.0.insert((x, y), entity);
    }
}

fn create(
    mouse: Res<Input<MouseButton>>,
    window: Query<&Window>,
    cam: Query<(&Camera, &GlobalTransform)>,
    mut cells: ResMut<Cells>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    let window = window.single();
    let cam = cam.single();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
        };
    let pos = cam.0.viewport_to_world_2d(cam.1, cursor_pos);
    let Some(pos) = pos else {
        return;
    };

    let x = pos.x.round() as i32;
    let y = pos.y.round() as i32;

    cells.0.insert((x, y), true);
}

fn step(mut next_ev: EventReader<NextStepEvent>, mut cells: ResMut<Cells>) {
    if next_ev.is_empty() {
        return;
    }
    next_ev.clear();

    let mut changes = HashMap::new();

    for ((x, y), alive) in &cells.0 {
        let live_neigbours = [
            (x + 1, y + 1),
            (x + 1, y + 0),
            (x + 1, y - 1),
            (x + 0, y - 1),
            (x + 0, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x - 1, y + 0),
        ]
        .iter()
        .filter(|p| cells.0.get(*p) == Some(&true))
        .count();
        if *alive == true {
            println!("{},{} = {}", x, y, live_neigbours);
            if live_neigbours < 2 || live_neigbours > 3 {
                changes.insert((*x, *y), false);
            }
        } else {
            if live_neigbours == 3 {
                changes.insert((*x, *y), true);
            }
        }
    }

    for (k, v) in changes.iter() {
        println!("{:?} {:?}", k, v);
        cells.0.insert(*k, *v);
    }

    println!("update ....................");
}

fn pause_play(
    keys: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        println!("space");
        if game_state.get() == &GameState::Play {
            next_state.set(GameState::Pause);
        } else {
            next_state.set(GameState::Play);
        }
    }
}
fn manuel_step(keys: Res<Input<KeyCode>>, mut next_ev: EventWriter<NextStepEvent>) {
    if keys.just_pressed(KeyCode::Right) {
        next_ev.send(NextStepEvent);
    }
}

fn render(mut q_cells: Query<(&mut Visibility, &Cell)>, cells: Res<Cells>) {
    for (mut vis, Cell { x, y }) in &mut q_cells {
        let cell = cells.0.get(&(*x, *y));
        *vis = if cell == Some(&true) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn run_timer(
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut next_ev: EventWriter<NextStepEvent>,
) {
    game_timer.0.tick(time.delta());

    if game_timer.0.just_finished() {
        next_ev.send(NextStepEvent);
    }
}
