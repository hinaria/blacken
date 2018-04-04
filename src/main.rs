#![windows_subsystem = "windows"]


extern crate winapi as win32;


use {
    std::time::Duration,

    win32::um::winuser::PostMessageW,
    win32::um::winuser::HWND_BROADCAST,
    win32::um::winuser::WM_SYSCOMMAND,
    win32::um::winuser::SC_MONITORPOWER,
};


fn main() {
    let delay = Duration::from_secs(1);

    // lparam values for `SC_MONITORPOWER`:
    //     -1: on
    //      1: low power
    //      2: off
    //
    // -- https://msdn.microsoft.com/en-us/library/windows/desktop/ms646360(v=vs.85).aspx

    std::thread::sleep(delay);
    unsafe { PostMessageW(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, 2); }
}
