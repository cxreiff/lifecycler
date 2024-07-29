use bevy::asset::embedded_asset;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let prefix = "src/";
    embedded_asset!(app, prefix, "../assets/base.glb");
    embedded_asset!(app, prefix, "../assets/bubble.ogg");
    embedded_asset!(app, prefix, "../assets/coral.glb");
    embedded_asset!(app, prefix, "../assets/fish.glb");
    embedded_asset!(app, prefix, "../assets/frame.glb");
    embedded_asset!(app, prefix, "../assets/gravel.glb");
    embedded_asset!(app, prefix, "../assets/off.ogg");
    embedded_asset!(app, prefix, "../assets/on.ogg");
    embedded_asset!(app, prefix, "../assets/rocks.glb");
    embedded_asset!(app, prefix, "../assets/skeleton.glb");
    embedded_asset!(app, prefix, "../assets/snail.glb");
    embedded_asset!(app, prefix, "../assets/starfish.glb");
    embedded_asset!(app, prefix, "../assets/tank.glb");
}
