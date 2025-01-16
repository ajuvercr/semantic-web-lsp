use bevy_ecs::prelude::*;
use bevy_ecs::schedule::ScheduleLabel;
use client::Client;
use components::{SemanticTokensDict, TypeHierarchy};
use systems::{
    basic_semantic_tokens, complete_class, complete_properties, defined_prefix_completion,
    derive_classes, derive_prefix_links, derive_properties, derive_shapes, extract_type_hierarchy,
    fetch_lov_properties, get_current_token, get_current_triple, hover_class, hover_property,
    hover_types, infer_types, keyword_complete, prefixes, semantic_tokens_system, validate_shapes,
};

/// Main language tower_lsp server implementation.
///
/// [`Backend`](struct@backend::Backend) implements [`LanguageServer`](tower_lsp::LanguageServer).
/// Each incoming request a schedule is ran on the main [`World`].
pub mod backend;

/// Handle platform specific implementations for fetching and spawning tasks.
pub mod client;

/// Defines all common [`Component`]s and [`Resource`]s
///
/// In this [`World`], [Entity]s are documents and [`Components`](`Component`) are derived from these documents.
/// Different [`System`]s derive new [`Components`](`Component`) from existing [`Components`](`Component`), that are added to
/// the [`Entity`].
/// For example, if [`Triples`](components::Triples) are defined, [systems::derive_classes] will
/// derive [`DefinedClass`](struct@systems::DefinedClass) from them and add them to the [`Entity`].
pub mod components;
/// Hosts all common features of the semantic language server.
pub mod features;
/// Defines common language traits
pub mod lang;
/// Commonly used RDF prefixes
pub mod ns;
pub mod prelude;
pub mod systems;
/// All token definitions, for all semantic languages
pub mod token;
/// Custom triple implementation.
pub mod triples;
/// Common utils
///
/// Includes range transformations between [`std::ops::Range`] and [`lsp_types::Range`].
/// And commonly used [`Spanned`](crate::prelude::Spanned).
pub mod utils;

/// Initializes a [`World`], including [`Resources`](`Resource`) and [`Schedules`].
/// All systems defined in [`crate`] are added to the [`World`].
pub fn setup_schedule_labels<C: Client + Resource>(world: &mut World) {
    world.init_resource::<SemanticTokensDict>();
    world.init_resource::<TypeHierarchy<'static>>();

    let mut parse = Schedule::new(Parse);
    parse.add_systems((
        prefixes,
        systems::triples,
        derive_prefix_links.after(prefixes),
        derive_classes.after(systems::triples),
        derive_properties.after(systems::triples),
        fetch_lov_properties::<C>.after(prefixes),
        extract_type_hierarchy.after(systems::triples),
        infer_types.after(systems::triples),
        derive_shapes.after(systems::triples),
    ));
    world.add_schedule(parse);

    let mut completion = Schedule::new(Completion);
    println!("Setting completion systems");
    completion.add_systems((
        get_current_token,
        keyword_complete.after(get_current_token),
        get_current_triple.after(get_current_token),
        complete_class.after(get_current_triple),
        complete_properties.after(get_current_triple),
        defined_prefix_completion.after(get_current_token),
    ));
    world.add_schedule(completion);

    let mut hover = Schedule::new(Hover);
    hover.add_systems((
        infer_types,
        get_current_token,
        get_current_triple.after(get_current_token),
        hover_types
            .before(hover_class)
            .before(hover_property)
            .after(get_current_token)
            .after(infer_types),
        hover_class.after(get_current_token),
        hover_property.after(get_current_token),
    ));
    world.add_schedule(hover);

    let mut diagnostics = Schedule::new(Diagnostics);
    diagnostics.add_systems((systems::undefined_prefix,));
    world.add_schedule(diagnostics);

    let mut on_save = Schedule::new(OnSave);
    on_save.add_systems((validate_shapes,));
    world.add_schedule(on_save);

    let mut prepare_rename = Schedule::new(PrepareRename);
    prepare_rename.add_systems((
        get_current_token,
        systems::prepare_rename.after(get_current_token),
    ));
    world.add_schedule(prepare_rename);

    let mut rename = Schedule::new(Rename);
    rename.add_systems((get_current_token, systems::rename.after(get_current_token)));
    world.add_schedule(rename);

    world.add_schedule(Schedule::new(Tasks));
    world.add_schedule(Schedule::new(Format));
    let inlay = Schedule::new(Inlay);
    // inlay.add_systems(inlay_triples);
    world.add_schedule(inlay);

    let mut semantic_tokens = Schedule::new(systems::SemanticTokensSchedule);
    semantic_tokens.add_systems((
        basic_semantic_tokens,
        semantic_tokens_system.after(basic_semantic_tokens),
    ));
    world.add_schedule(semantic_tokens);
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

/// [`ScheduleLabel`] related to the Hover schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Hover;

/// [`ScheduleLabel`] related to the Parse schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Parse;

/// [`ScheduleLabel`] related to the Completion schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Completion;

/// [`ScheduleLabel`] related to the Diagnostics schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Diagnostics;

/// [`ScheduleLabel`] related to the OnSave schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OnSave;

/// [`ScheduleLabel`] related to the Tasks schedule
/// This schedule is used for async tasks, things that should be done at some point.
///
/// For example [`systems::handle_tasks`] spawns command queues sent with
/// [`CommandSender`](components::CommandSender)
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Tasks;

/// [`ScheduleLabel`] related to the Format schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Format;

/// [`ScheduleLabel`] related to the PrepareRename schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrepareRename;

/// [`ScheduleLabel`] related to the Rename schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Rename;

/// [`ScheduleLabel`] related to the Inlay schedule
#[derive(ScheduleLabel, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Inlay;
