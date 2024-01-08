use crate::animation::*;
use bevy::prelude::*;

/// プレイヤー
#[derive(Component, Default, Debug)]
pub struct Player;

/// プレイヤーアニメーション
#[derive(Component, Default, Debug)]
pub struct PlayerAnimation {
    pub idle: SpriteAnimation,
    pub left_run: SpriteAnimation,
    pub right_run: SpriteAnimation,
    pub up_run: SpriteAnimation,
    pub down_run: SpriteAnimation,
}

/// プレイヤー関連のコンボーネントバンドル
#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite: SpriteSheetBundle,
    pub animation: PlayerAnimation,
}