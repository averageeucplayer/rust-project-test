#[cfg(test)]
use tauri::State;

#[cfg(test)]
pub fn to_state_unsafe<'r, T: Send + Sync + 'static>(input: &'r T) -> State<'r, T> {
    #[allow(dead_code)]
    pub struct FakeTauriState<'r, T: Send + Sync + 'static>(&'r T);

    let fake_state = FakeTauriState(input);
    unsafe { std::mem::transmute(fake_state) }
}