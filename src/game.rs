use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub i32);

#[derive(Component)]
pub struct Move(f32);

#[derive(Component)]
pub struct Reset(f32, f32);

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Update, (start_game).run_if(in_state(GameState::MainMenu)))
            .add_systems(Update, (score_log).run_if(in_state(GameState::Game)));
    }
}

fn start_game(keys: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("start game");
        next_state.set(GameState::Game);
    }
}

fn score_log(score: Res<Score>) {
    if score.is_changed() {
        println!("score: {}", score.0);
    }
}
