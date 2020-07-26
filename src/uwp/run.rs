use bindings::{
    windows::{
        application_model::core::{
            CoreApplication, CoreApplicationView, IFrameworkViewSource, IFrameworkView,
        },
        ui::core::{
            CoreDispatcher, CoreWindow, CoreProcessEventsOption, WindowSizeChangedEventArgs, 
            PointerEventArgs
        }
    }
};
use winrt::TryInto;
use crate::uwp::app::App;

pub fn run() -> winrt::Result<()> {
    let app = App::new()?;
    let view_source: IFrameworkViewSource = app.try_into()?;
    CoreApplication::run(&view_source)?;
    Ok(())
}