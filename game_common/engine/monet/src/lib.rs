// temporary fix for https://github.com/glium/glium/issues/1620
#![cfg_attr(feature = "cargo-clippy", allow(forget_copy))]

extern crate descartes;
#[macro_use]
pub extern crate glium;
extern crate kay;
extern crate compact;
#[macro_use]
extern crate compact_macros;
extern crate fnv;
extern crate itertools;
extern crate lyon_tessellation;
#[macro_use]
extern crate serde_derive;

mod mesh;
mod mesh_actors;
mod renderer;
mod render_context;
mod scene;

pub use glium::backend::glutin::Display;

pub use mesh::{Mesh, Batch, Vertex, Instance};
pub use mesh_actors::{Grouper, GrouperID, GrouperIndividual, GrouperIndividualID};
pub use renderer::{setup, Renderer, RendererID, Renderable, RenderableID, TargetProvider,
                   TargetProviderID, Movement, EyeListener, EyeListenerID, ProjectionRequester,
ProjectionRequesterID};
pub use render_context::RenderContext;
pub use scene::{Eye, Scene, SceneDescription};
