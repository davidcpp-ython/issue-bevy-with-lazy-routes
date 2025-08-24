use leptos::prelude::*;

#[component]
pub fn BevyCanvas() -> impl IntoView {
    // Use a runtime check to determine if we're in the browser
    let is_browser = RwSignal::new(false);
    
    // This will only run on the client side after hydration
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            is_browser.set(true);
        });
    }

    view! {
        <div class="bevy-canvas-container">
            <Show
                when=move || is_browser.get()
                fallback=|| view! {
                    <div class="bevy-canvas-placeholder" style="width: 800px; height: 600px; background: #1a1a1a;">
                        <p style="color: white; text-align: center; padding-top: 280px;">
                            "Bevy Canvas (Client-side only)"
                        </p>
                    </div>
                }
            >
                {
                    #[cfg(target_arch = "wasm32")]
                    {
                        view! { <BevyCanvasRuntime /> }
                    }
                    
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        view! { <div/> }
                    }
                }
            </Show>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[component]
pub fn BevyCanvasRuntime() -> impl IntoView {
    use leptos_bevy_canvas::prelude::*;
    
    view! {
        <div style="position: relative; width: 800px; height: 600px;">
            <BevyCanvas
                init=move || init_bevy_app()
            />
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn init_bevy_app() -> bevy::prelude::App {
    use bevy::prelude::*;
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            canvas: Some("#bevy_canvas".into()),
            fit_canvas_to_parent: true,
            prevent_default_event_handling: true,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup_scene)
    .add_systems(Update, rotate_shapes);
    
    app
}

#[cfg(target_arch = "wasm32")]
fn setup_scene(mut commands: bevy::prelude::Commands) {
    use bevy::prelude::*;
    
    // Camera
    commands.spawn(Camera2d::default());

    // Spawn some shapes
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            color: Color::srgb(0.25, 0.75, 0.45),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 0.0),
        RotatingShape { speed: 1.0 },
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(80.0, 80.0)),
            color: Color::srgb(0.75, 0.25, 0.45),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingShape { speed: -1.5 },
    ));

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(120.0, 60.0)),
            color: Color::srgb(0.45, 0.45, 0.75),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
        RotatingShape { speed: 0.75 },
    ));
}

#[cfg(target_arch = "wasm32")]
#[derive(bevy::prelude::Component)]
struct RotatingShape {
    speed: f32,
}

#[cfg(target_arch = "wasm32")]
fn rotate_shapes(mut query: bevy::prelude::Query<(&mut bevy::prelude::Transform, &RotatingShape)>, time: bevy::prelude::Res<bevy::prelude::Time>) {
    for (mut transform, shape) in query.iter_mut() {
        transform.rotate_z(shape.speed * time.delta_secs());
    }
}