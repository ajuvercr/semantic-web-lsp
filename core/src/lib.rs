use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use client::Client;
use components::{SemanticTokensDict, TypeHierarchy};
use feature::{completion, diagnostics, format, hover, inlay, parse, rename, save, semantic};

/// Main language tower_lsp server implementation.
///
/// [`Backend`](struct@backend::Backend) implements [`LanguageServer`](tower_lsp::LanguageServer).
/// Each incoming request a schedule is ran on the main [`World`].
pub mod backend;

/// Handle platform specific implementations for fetching and spawning tasks.
pub mod client;
/// Common utils
///
/// Includes range transformations between [`std::ops::Range`] and [`lsp_types::Range`].
/// And commonly used [`Spanned`](crate::prelude::Spanned).
pub mod util;

/// Defines all common [`Component`]s and [`Resource`]s
///
/// In this [`World`], [Entity]s are documents and [`Components`](`Component`) are derived from these documents.
/// Different [`System`]s derive new [`Components`](`Component`) from existing [`Components`](`Component`), that are added to
/// the [`Entity`].
/// For example, if [`Triples`](components::Triples) are defined, [systems::derive_classes] will
/// derive [`DefinedClass`](struct@systems::DefinedClass) from them and add them to the [`Entity`].
pub mod components;
/// Hosts all common features of the semantic language server.
pub mod feature;
/// Defines common language traits
pub mod lang;
pub mod prelude;
pub mod systems;

/// Initializes a [`World`], including [`Resources`](`Resource`) and [`Schedules`].
/// All systems defined in [`crate`] are added to the [`World`].
pub fn setup_schedule_labels<C: Client + Resource>(world: &mut World) {
    world.init_resource::<SemanticTokensDict>();
    world.init_resource::<TypeHierarchy<'static>>();

    parse::setup_schedule::<C>(world);
    hover::setup_schedule(world);
    completion::setup_schedule(world);
    rename::setup_schedules(world);
    diagnostics::setup_schedule(world);
    save::setup_schedule(world);
    format::setup_schedule(world);
    inlay::setup_schedule(world);
    semantic::setup_world(world);

    world.add_schedule(Schedule::new(Tasks));
}

/// Event triggers when a document is opened
///
/// Example
/// ```rust
/// # use lsp_core::components::DynLang;
/// # use lsp_core::CreateEvent;
/// # use lsp_core::lang::LangHelper;
/// # use bevy_ecs::prelude::{Commands, Trigger, World, Component};
///
/// #[derive(Component)]
/// pub struct TurtleLang;
///
/// #[derive(Debug)]
/// pub struct TurtleHelper;
/// impl LangHelper for TurtleHelper {
///     fn keyword(&self) -> &[&'static str] {
///         &[
///             "@prefix",
///             "@base",
///             "a",
///         ]
///     }
/// }
///
/// let mut world = World::new();
/// // This example tells the ECS system that the document is Turtle,
/// // adding Turtle specific components
/// world.observe(|trigger: Trigger<CreateEvent>, mut commands: Commands| {
///     match &trigger.event().language_id {
///         Some(x) if x == "turtle" => {
///             commands
///                 .entity(trigger.entity())
///                 .insert((TurtleLang, DynLang(Box::new(TurtleHelper))));
///             return;
///         }
///         _ => {}
///     }
///     if trigger.event().url.as_str().ends_with(".ttl") {
///         commands
///             .entity(trigger.entity())
///             .insert((TurtleLang, DynLang(Box::new(TurtleHelper))));
///         return;
///     }
/// });
/// ```
///
#[derive(Event)]
pub struct CreateEvent {
    pub url: lsp_types::Url,
    pub language_id: Option<String>,
}

/// [`ScheduleLabel`] related to the Tasks schedule
/// This schedule is used for async tasks, things that should be done at some point.
///
/// For example [`systems::handle_tasks`] spawns command queues sent with
/// [`CommandSender`](components::CommandSender)
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Tasks;
