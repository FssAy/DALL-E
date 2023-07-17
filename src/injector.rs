use tauri::{Runtime, Window};


#[tauri::command]
pub fn inject<R: Runtime>(window: Window<R>) -> Result<(), tauri::Error> {
    window.eval(crate::js::I18_STRINGS)?;
    window.eval(crate::js::EXPLOIT)?;
    Ok(())
}
