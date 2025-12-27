use std::ptr::NonNull;

use windows_sys::Win32::Storage::FileSystem::{
    FILE_TYPE_DISK, FILE_TYPE_PIPE, GetFileType,
};
use windows_sys::Win32::System::Console::{
    ATTACH_PARENT_PROCESS, AttachConsole, GetStdHandle,
    WriteConsoleW, STD_OUTPUT_HANDLE,
};

fn main() {
    let message = "Hello from the console!\r\n";
	let message2 = "Hello from standard out!\n";

    unsafe {
        // Try to get stdout handle
        let stdout_handle = NonNull::new(GetStdHandle(STD_OUTPUT_HANDLE));

        // Determine whether we should attach
        let should_attach = stdout_handle.is_none_or(|mut handle| {
            // If the handle exists, check its type
            let file_type = GetFileType(handle.as_mut());

            // If redirected to file or pipe → do NOT attach
            !matches!(file_type, FILE_TYPE_DISK | FILE_TYPE_PIPE)
        });

        // Attempt to attach if needed
        if should_attach {
            let _ = AttachConsole(ATTACH_PARENT_PROCESS);
        }

        // Re-fetch the handle after possible attach
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);

        // Prepare UTF-16 buffer
        let wide: Vec<u16> = message.encode_utf16().collect();
        let mut written = 0;

        // Try console write
        let ok = WriteConsoleW(
            handle,
            wide.as_ptr() as *const _,
            wide.len() as u32,
            &mut written,
            core::ptr::null_mut(),
        );

        // If console write failed, fall back to stream output
        if ok == 0 {
            print!("{message2}");
        }
    }
}