use super::ffi::{DM_BRIDGE_BUFFER_TOO_SMALL, DM_BRIDGE_OK};

pub fn to_wide_nul(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn call_wide_output<F>(mut call: F) -> (i32, String)
where
    F: FnMut(*mut u16, u32, *mut u32) -> i32,
{
    let mut capacity = 256usize;

    loop {
        let mut buffer = vec![0u16; capacity];
        let mut output_len = 0u32;
        let status = call(
            buffer.as_mut_ptr(),
            capacity as u32,
            &mut output_len as *mut u32,
        );

        // DmBridge 采用调用方 buffer 模式。
        // 输入：UTF-16 buffer 和容量。
        // 输出：成功文本，或按 out_len 扩容后重试。
        // 边界：out_len 不含 NUL，capacity 是 u16 数量，不是字节数。
        if status == DM_BRIDGE_BUFFER_TOO_SMALL && output_len as usize + 1 > capacity {
            capacity = output_len as usize + 1;
            continue;
        }

        let safe_len = (output_len as usize).min(buffer.len());
        let text = if status == DM_BRIDGE_OK || safe_len > 0 {
            String::from_utf16_lossy(&buffer[..safe_len])
        } else {
            String::new()
        };
        return (status, text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_string_has_trailing_nul() {
        let value = to_wide_nul("abc");

        assert_eq!(value, vec![97, 98, 99, 0]);
    }
}
