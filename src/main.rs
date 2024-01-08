use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme}
};
use service::player::*;

fn main() {
    App::new()
        .add_plugins(
            // アプリケーションのウィンドウ設定
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Elepaz".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
        )
        // Startupはアプリ起動時に1回読み込まれる。
        .add_systems(Startup, (install_camera, create_player))
        // Updateはフレーム単位で読み込まれる。
        .add_systems(Update, update_player)
        // escでアプリ終了させる。
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

//カメラの設置
fn install_camera(mut cmds: Commands) {
    // 2Dカメラの読み込み
    cmds.spawn(Camera2dBundle::default());
}

