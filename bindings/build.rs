winrt::build!(
    dependencies
        os
    types
        windows::application_model::core::{
            CoreApplication, CoreApplicationView, IFrameworkViewSource, IFrameworkView,
        }
        windows::foundation::numerics::{Vector2, Vector3}
        windows::foundation::TimeSpan
        windows::graphics::SizeInt32
        windows::system::DispatcherQueueController
        windows::ui::composition::{
            AnimationIterationBehavior,
            CompositionBatchTypes,
            CompositionBorderMode,
            CompositionColorBrush,
            CompositionGeometry,
            CompositionShape,
            CompositionSpriteShape,
            CompositionTarget,
            Compositor,
            ContainerVisual,
            SpriteVisual,
        }
        windows::ui::composition::desktop::DesktopWindowTarget
        windows::ui::Colors
        windows::ui::core::{
            CoreDispatcher, CoreWindow, CoreProcessEventsOption, WindowSizeChangedEventArgs, 
            PointerEventArgs
        }
);

fn main() {
    build();
}
