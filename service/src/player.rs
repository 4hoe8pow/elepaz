use core::{animation::*, player::*};

use bevy::prelude::*;

/// プレイヤーの生成
pub fn create_player(
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
                index: SpriteAnimationIndex {
                    first: 12,
                    last: 17,
                },
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
                index: SpriteAnimationIndex {
                    first: 18,
                    last: 23,
                },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
        },
        ..default()
    },));
}

/// プレイヤーの更新
pub fn update_player(
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