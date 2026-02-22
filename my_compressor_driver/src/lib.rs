use wdk_sys::{
    PDRIVER_OBJECT, PCUNICODE_STRING, NTSTATUS, DRIVER_OBJECT,
    FltRegisterFilter, FltStartFiltering, FLT_REGISTRATION,
    FLT_OPERATION_REGISTRATION, IRP_MJ_READ, IRP_MJ_WRITE,
    FLT_CONTEXT_REGISTRATION, FLT_REGISTRATION_FLAGS_DO_NOT_SUPPORT_SERVICE_STOP,
    FLT_PREOP_CALLBACK_STATUS, FLT_POSTOP_CALLBACK_STATUS,
    FLT_CALLBACK_DATA, FLT_RELATED_OBJECTS
};
use lexivault_lib::{HybridCompressor, BlockCompressor, CompressionAlgorithm};

extern crate wdk_panic;
extern crate wdk_alloc;
use wdk_alloc::WdkAllocator;

mod ntfs;
use ntfs::{BlockMapTable, BlockIndexEntry};

#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

static mut FILTER_HANDLE: *mut wdk_sys::_FLT_FILTER = core::ptr::null_mut();

// Placeholder for custom Reparse Tag
const COMPRESSOR_REPARSE_TAG: u32 = 0x80000023; 

const CALLBACKS: &[FLT_OPERATION_REGISTRATION] = &[
    FLT_OPERATION_REGISTRATION {
        MajorFunction: IRP_MJ_READ as u8,
        Flags: 0,
        PreOperation: Some(pre_read),
        PostOperation: Some(post_read),
        Reserved1: core::ptr::null_mut(),
    },
    FLT_OPERATION_REGISTRATION {
        MajorFunction: IRP_MJ_WRITE as u8,
        Flags: 0,
        PreOperation: Some(pre_write),
        PostOperation: None,
        Reserved1: core::ptr::null_mut(),
    },
    FLT_OPERATION_REGISTRATION {
        MajorFunction: 0xFF,
        Flags: 0,
        PreOperation: None,
        PostOperation: None,
        Reserved1: core::ptr::null_mut(),
    },
];

const REGISTRATION: FLT_REGISTRATION = FLT_REGISTRATION {
    Size: core::mem::size_of::<FLT_REGISTRATION>() as u16,
    Version: wdk_sys::FLT_REGISTRATION_VERSION as u16,
    Flags: FLT_REGISTRATION_FLAGS_DO_NOT_SUPPORT_SERVICE_STOP,
    ContextRegistration: core::ptr::null(),
    OperationRegistration: CALLBACKS.as_ptr(),
    FilterUnloadCallback: Some(unload),
    InstanceSetupCallback: None,
    InstanceQueryTeardownCallback: None,
    InstanceTeardownStartCallback: None,
    InstanceTeardownCompleteCallback: None,
    GenerateFileNameCallback: None,
    NormalizeNameComponentCallback: None,
    TransactionNotificationCallback: None,
    NormalizeContextCleanupCallback: None,
    #[cfg(target_version = "vista")]
    SectionNotificationCallback: None,
};

#[no_mangle]
pub unsafe extern "system" fn driver_entry(
    driver: &mut DRIVER_OBJECT,
    _registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    let status = FltRegisterFilter(driver as *mut _, &REGISTRATION, &mut FILTER_HANDLE);
    if status >= 0 {
        let status = FltStartFiltering(FILTER_HANDLE);
        if status < 0 {
            wdk_sys::FltUnregisterFilter(FILTER_HANDLE);
        }
    }
    status
}

unsafe extern "system" fn unload(_flags: u32) -> NTSTATUS {
    wdk_sys::FltUnregisterFilter(FILTER_HANDLE);
    0 
}

unsafe extern "system" fn pre_read(
    data: *mut FLT_CALLBACK_DATA,
    _pc: *mut FLT_RELATED_OBJECTS,
    _completion_context: *mut *mut core::ffi::c_void,
) -> FLT_PREOP_CALLBACK_STATUS {
    let iopb = (*data).Iopb;
    
    // In a real minifilter, we'd check if the file object has our stream context.
    // For this demonstration, we'll mark it for post-processing if it's a candidate for decompression.
    
    // Check if the read is paging I/O or cached.
    // Transparent compression usually handles Non-cached I/O to the FSD.
    
    wdk_sys::FLT_PREOP_SUCCESS_WITH_CALLBACK
}

unsafe extern "system" fn post_read(
    data: *mut FLT_CALLBACK_DATA,
    _pc: *mut FLT_RELATED_OBJECTS,
    _completion_context: *mut core::ffi::c_void,
    _flags: u32,
) -> FLT_POSTOP_CALLBACK_STATUS {
    if !(*data).IoStatus.Status >= 0 {
        return wdk_sys::FLT_POSTOP_FINISHED_PROCESSING;
    }

    // This is where decompression happens.
    // data.Iopb.Parameters.Read.ReadBuffer contains the "on-disk" bytes.
    // We decompress them into the original user buffer.
    
    let read_params = &(*(*data).Iopb).Parameters.Read;
    let bytes_read = (*data).IoStatus.Information;
    
    if bytes_read == 0 {
        return wdk_sys::FLT_POSTOP_FINISHED_PROCESSING;
    }

    // 1. Logically, we would look up the block compression algorithm from the file's metadata.
    // 2. Here, assume Zstd for demonstration.
    let compressor = HybridCompressor::new();
    
    // Get the buffer. Minifilters handle different buffer types (MdlAddress vs UserBuffer).
    let buffer = if !(*data).Iopb.Parameters.Read.MdlAddress.is_null() {
        wdk_sys::MmGetSystemAddressForMdlSafe((*data).Iopb.Parameters.Read.MdlAddress, 16 /* NormalPagePriority */)
    } else {
        (*data).Iopb.Parameters.Read.ReadBuffer
    };

    if buffer.is_null() {
        return wdk_sys::FLT_POSTOP_FINISHED_PROCESSING;
    }

    let compressed_slice = core::slice::from_raw_parts(buffer as *const u8, bytes_read as usize);
    
    // Decompression (in a real driver, we'd use a temporary buffer and copy back to avoid corruption).
    if let Ok(decompressed) = compressor.decompress(compressed_slice, CompressionAlgorithm::ZstdFast) {
        let copy_len = core::cmp::min(decompressed.len(), read_params.Length as usize);
        core::ptr::copy_nonoverlapping(decompressed.as_ptr(), buffer as *mut u8, copy_len);
        (*data).IoStatus.Information = copy_len as u64;
    }

    wdk_sys::FLT_POSTOP_FINISHED_PROCESSING
}

unsafe extern "system" fn pre_write(
    data: *mut FLT_CALLBACK_DATA,
    _pc: *mut FLT_RELATED_OBJECTS,
    _completion_context: *mut *mut core::ffi::c_void,
) -> FLT_PREOP_CALLBACK_STATUS {
    if ((*data).Iopb).OperationFlags & wdk_sys::SL_WRITE_THROUGH != 0 {
        // Handle sync/async complexities...
    }

    let read_params = &(*(*data).Iopb).Parameters.Write;
    let bytes_to_write = read_params.Length;

    if bytes_to_write == 0 {
        return wdk_sys::FLT_PREOP_SUCCESS_NO_CALLBACK;
    }

    // 1. Get User Buffer
    let buffer = if !(*data).Iopb.Parameters.Write.MdlAddress.is_null() {
        wdk_sys::MmGetSystemAddressForMdlSafe((*data).Iopb.Parameters.Write.MdlAddress, 16)
    } else {
        (*data).Iopb.Parameters.Write.WriteBuffer
    };

    if buffer.is_null() {
        return wdk_sys::FLT_PREOP_SUCCESS_NO_CALLBACK;
    }

    let data_slice = core::slice::from_raw_parts(buffer as *const u8, bytes_to_write as usize);

    // 2. Compress using Hybrid Engine
    let compressor = HybridCompressor::new();
    if let Ok(compressed) = compressor.compress(data_slice, CompressionAlgorithm::ZstdFast) {
        // 3. Optimization: If compressed data isn't significantly smaller, don't compress.
        if compressed.len() >= data_slice.len() {
             return wdk_sys::FLT_PREOP_SUCCESS_NO_CALLBACK;
        }

        // 4. In a real implementation:
        // - We would swap the buffer with 'compressed'.
        // - Update the ADS index with the new BlockIndexEntry.
        // - Adjust the Iopb.Parameters.Write.Length.
        // - Return FLT_PREOP_SUCCESS_NO_CALLBACK to let the write proceed down.
    }

    wdk_sys::FLT_PREOP_SUCCESS_NO_CALLBACK
}

unsafe extern "system" fn fs_control(
    _data: *mut FLT_CALLBACK_DATA,
    _pc: *mut FLT_RELATED_OBJECTS,
    _completion_context: *mut *mut core::ffi::c_void,
) -> FLT_PREOP_CALLBACK_STATUS {
    // Handle IOCTL_MARK_AS_COMPRESSED etc.
    wdk_sys::FLT_PREOP_SUCCESS_NO_CALLBACK
}

