use windows_sys::Win32::System::Console::{
    GetStdHandle, WriteConsoleW, STD_OUTPUT_HANDLE,
};

fn main() {
    let message = "Hello from the console!";
	let message2 = "Hello from standard out!";

    // Convert to UTF-16 for WriteConsoleW
    let wide: Vec<u16> = message.encode_utf16().collect();

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut written = 0;
        let result = WriteConsoleW(
            handle,
            wide.as_ptr() as *const _,
            wide.len() as u32,
            &mut written,
            core::ptr::null_mut(),
        );

        if result == 0
		{
            // On failure, macro the message to stdout
            print!("{message2}");
        }
    }
}
