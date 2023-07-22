use std::ffi::c_char;

extern "C" {
    fn segappend_create_segment(
        binary_path: *const c_char,
        segment_name: *const c_char,
        data: *const u8,
        size: usize,
        output_path: *const c_char,
    ) -> u32;

    fn segappend_load_segment(
        segment_name: *const c_char,
        data: *mut *const u8,
        size: *mut usize,
    ) -> u32;
}

#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum SegappendError {
    SegmentNotFound = 1,
    FileNotFound = 2,
    LinkeditNotFound = 3,
    CannotWrite = 4,
}

impl From<u32> for SegappendError {
    fn from(code: u32) -> Self {
        match code {
            1 => SegappendError::SegmentNotFound,
            2 => SegappendError::FileNotFound,
            3 => SegappendError::LinkeditNotFound,
            4 => SegappendError::CannotWrite,
            _ => panic!("Unknown error code: {}", code),
        }
    }
}

pub fn create(
    binary_path: &str,
    segment_name: &str,
    data: *const u8,
    size: usize,
    output_path: &str,
) -> Result<(), SegappendError> {
    let binary_path = std::ffi::CString::new(binary_path).unwrap();
    let segment_name = std::ffi::CString::new(segment_name).unwrap();
    let output_path = std::ffi::CString::new(output_path).unwrap();
    let result = unsafe {
        segappend_create_segment(
            binary_path.as_ptr(),
            segment_name.as_ptr(),
            data,
            size,
            output_path.as_ptr(),
        )
    };
    if result == 0 {
        Ok(())
    } else {
        Err(SegappendError::from(result))
    }
}

pub fn load(segment_name: &str) -> Result<*const u8, SegappendError> {
    let mut data: *const u8 = std::ptr::null();
    let mut size: usize = 0;
    let segment_name = std::ffi::CString::new(segment_name).unwrap();
    let result = unsafe { segappend_load_segment(segment_name.as_ptr(), &mut data, &mut size) };
    if result == 0 {
        Ok(data)
    } else {
        Err(SegappendError::from(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let segment_name = "hello";
        let result = load(segment_name);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SegappendError::SegmentNotFound);
    }
}
