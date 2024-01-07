use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};

/// プレイヤー
#[derive(Component, Default, Debug)]
struct Player;

/// プレイヤーアニメーション
#[derive(Component, Default, Debug)]
struct PlayerAnimation {
    idle: SpriteAnimation,
    left_run: SpriteAnimation,
    right_run: SpriteAnimation,
    up_run: SpriteAnimation,
    down_run: SpriteAnimation,
}

/// プレイヤー関連のコンボーネントバンドル
#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    sprite: SpriteSheetBundle,
    animation: PlayerAnimation,
}

/// スプライトアニメーション
#[derive(Component, Default, Debug)]
struct SpriteAnimation {
    is_playing: bool,
    index: SpriteAnimationIndex,
    timer: SpriteAnimationTimer,
}

/// スプライトアニメーションのインデックス
#[derive(Component, Default, Debug)]
struct SpriteAnimationIndex {
    first: usize,
    last: usize,
}

/// スプライトアニメーションのタイマー(タプル構造体)
#[derive(Component, Default, Debug, Deref, DerefMut)]
struct SpriteAnimationTimer(Timer);

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
        .add_systems(Startup, create_player)
        // Updateはフレーム単位で読み込まれる。
        .add_systems(Update, update_player)
        // escでアプリ終了させる。
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

/// プレイヤーの生成
fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // プレイヤーのテクスチャー画像を読み込む。
    let texture_handle: Handle<Image> = asset_server.load("Alex_run_16x16.png");
    // プレイヤーテクスチャー画像のアトラス形式にする。
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 32.0), 24, 1, None, None);
    // プライヤーテクスチャー画像のアトラスを取り扱いできるようにする。
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // 2Dカメラの読み込み
    commands.spawn(Camera2dBundle::default());

    // プレイヤーの読み込み
    commands.spawn((PlayerBundle {
        // プレイヤーのスプライト追加
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        },
        // 用意したプレイヤーアニメーションコンポーネントの追加
        animation: PlayerAnimation {
            // 待機状態のアニメーション追加
            idle: SpriteAnimation {
                index: SpriteAnimationIndex { first: 0, last: 5 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ..default()
            },
            // 移動状態のアニメーション追加
            right_run: SpriteAnimation {
                index: SpriteAnimationIndex { first: 0, last: 5 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
            // 移動状態のアニメーション追加
            left_run: SpriteAnimation {
                index: SpriteAnimationIndex { first: 12, last: 17 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
            //
            up_run: SpriteAnimation {
                index: SpriteAnimationIndex { first: 6, last: 11 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
            //
            down_run: SpriteAnimation {
                index: SpriteAnimationIndex { first: 18, last: 23 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
        },
        ..default()
    },));
}

/// プレイヤーの更新
fn update_player(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<
        (
            &mut Transform,
            &mut TextureAtlasSprite,
            &mut PlayerAnimation,
        ),
        With<Player>,
    >,
) {
    // プレイヤーのEntityを取得(PlayerBundleの中にあるPlayerコンポーネントは1つなのでsingle_mutで取得)
    let (mut transform, mut sprite, mut anim) = player_query.single_mut();

    // 左キーで左に移動、右キーで右に移動させる。
    if keyboard_input.pressed(KeyCode::Left) {
        // 左に移動
        transform.translation.x -= 2.0;
        // 再生されている可能性があるアニメーションの停止
        anim.idle.stop();
        anim.right_run.stop();
        anim.up_run.stop();
        anim.down_run.stop();
        // 左に移動のアニメーション再生
        anim.left_run.play(sprite.as_mut(), time);
    } else if keyboard_input.pressed(KeyCode::Right) {
        // 右に移動
        transform.translation.x += 2.0;
        // 再生されている可能性があるアニメーションの停止
        anim.idle.stop();
        anim.left_run.stop();
        anim.up_run.stop();
        anim.down_run.stop();
        // 右に移動のアニメーション再生
        anim.right_run.play(sprite.as_mut(), time);
    } else if keyboard_input.pressed(KeyCode::Up) {
        // 上に移動
        transform.translation.y += 2.0;
        // 再生されている可能性があるアニメーションの停止
        anim.idle.stop();
        anim.left_run.stop();
        anim.right_run.stop();
        anim.down_run.stop();
        // 右に移動のアニメーション再生
        anim.up_run.play(sprite.as_mut(), time);
    } else if keyboard_input.pressed(KeyCode::Down) {
        // 右に移動
        transform.translation.y -= 2.0;
        // 再生されている可能性があるアニメーションの停止
        anim.idle.stop();
        anim.up_run.stop();
        anim.right_run.stop();
        anim.left_run.stop();
        // 右に移動のアニメーション再生
        anim.down_run.play(sprite.as_mut(), time);
    } else {
        // 再生されている可能性があるアニメーションの停止
        anim.left_run.stop();
        anim.right_run.stop();
        // 停止アニメーションの再生
        anim.idle.play(sprite.as_mut(), time);
    }
}

// スプライトアニメーションの構造体にstopとplayの機能を追加
impl SpriteAnimation {
    /// スプライトアニメーションの停止
    /// 次のアニメーションを再生する際は再生している可能性があるものを停止する。
    fn stop(&mut self) {
        self.is_playing = false;
    }
    /// スプライトアニメーションの再生
    fn play(&mut self, sprite: &mut TextureAtlasSprite, time: Res<Time>) {
        // 停止中だった場合に再生中をfalseとする。
        if !self.is_playing {
            sprite.index = self.index.first;
            self.is_playing = true;
        }
        // アニメタイマーをtime.delta()秒進める。
        // Timer::from_seconds(0.1, TimerMode::Repeating)と設定しているため、0.1秒経つとjust_finished()がtrueになる。
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            sprite.index = if sprite.index == self.index.last {
                self.index.first
            } else {
                sprite.index + 1
            };
        }
    }
}
