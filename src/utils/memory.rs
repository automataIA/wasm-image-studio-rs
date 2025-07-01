//! Memory management utilities for WebAssembly
//!
//! This module provides memory management utilities for WebAssembly applications,
//! including a buffer pool for efficient memory reuse and WebAssembly-specific
//! memory allocation/deallocation functions.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

/// Maximum number of buffers to keep in the pool for each size
const MAX_BUFFERS_PER_SIZE: usize = 10;

/// Buffer pool for reusing memory allocations to reduce memory fragmentation
/// and improve performance by avoiding frequent allocations/deallocations.
static BUFFER_POOL: Lazy<Mutex<HashMap<usize, Vec<Vec<u8>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Get a buffer from the pool or create a new one if none is available.
///
/// This function attempts to reuse an existing buffer of the requested size
/// from the pool. If no suitable buffer is available, it creates a new one.
///
/// # Arguments
///
/// * `size` - The size of the buffer in bytes
///
/// # Returns
///
/// A vector with the requested size, either from the pool or newly allocated
pub fn get_buffer(size: usize) -> Vec<u8> {
    // Try to get a lock on the buffer pool
    let pool_result = BUFFER_POOL.lock();
    
    // Handle potential lock poisoning gracefully
    let mut pool = match pool_result {
        Ok(guard) => guard,
        Err(_) => {
            // If the lock is poisoned, just create a new buffer
            // This is safer than panicking
            return vec![0; size];
        }
    };

    // Try to get a buffer of the requested size from the pool
    if let Some(buffers) = pool.get_mut(&size) {
        if let Some(buffer) = buffers.pop() {
            return buffer;
        }
    }

    // Create a new buffer if none available
    vec![0; size]
}

/// Return a buffer to the pool for reuse.
///
/// This function adds the buffer back to the pool for future reuse,
/// maintaining a maximum number of buffers per size to avoid memory bloat.
///
/// # Arguments
///
/// * `buffer` - The buffer to return to the pool
pub fn return_buffer(buffer: Vec<u8>) {
    let size = buffer.capacity();
    
    // Try to get a lock on the buffer pool
    let pool_result = BUFFER_POOL.lock();
    
    // Handle potential lock poisoning gracefully
    let mut pool = match pool_result {
        Ok(guard) => guard,
        Err(_) => {
            // If the lock is poisoned, just let the buffer drop
            return;
        }
    };

    // Add the buffer to the pool
    pool.entry(size).or_default().push(buffer);

    // Limit pool size to avoid memory bloat
    if let Some(buffers) = pool.get_mut(&size) {
        if buffers.len() > MAX_BUFFERS_PER_SIZE {
            buffers.truncate(MAX_BUFFERS_PER_SIZE);
        }
    }
}

/// Clear the buffer pool, releasing all memory.
///
/// This function removes all buffers from the pool, allowing them to be
/// deallocated by the Rust runtime.
pub fn clear_buffer_pool() {
    if let Ok(mut pool) = BUFFER_POOL.lock() {
        pool.clear();
    }
    // If the lock is poisoned, we can't clear the pool,
    // but we also don't want to panic
}

/// Allocate memory for use in WebAssembly.
///
/// This function allocates a buffer of the specified size and returns a pointer
/// to it. The memory is not automatically freed and must be explicitly
/// deallocated using `dealloc_memory`.
///
/// # Arguments
///
/// * `size` - The size of the memory to allocate in bytes
///
/// # Returns
///
/// A raw pointer to the allocated memory
#[wasm_bindgen]
pub fn alloc_memory(size: usize) -> *mut u8 {
    // Create a new buffer
    let mut buffer = get_buffer(size);

    // Get pointer to the buffer
    let ptr = buffer.as_mut_ptr();

    // Prevent the buffer from being dropped, transferring ownership to the caller
    std::mem::forget(buffer);

    ptr
}

/// Deallocate memory previously allocated with `alloc_memory`.
///
/// # Safety
///
/// This function is unsafe because it creates a Vec from a raw pointer.
/// The caller must ensure that:
/// 1. The pointer was previously returned by `alloc_memory`
/// 2. The size matches the original allocation size
/// 3. The memory has not been deallocated already
/// 4. No other code is currently using this memory
///
/// Failure to meet these conditions will result in undefined behavior,
/// potentially including memory corruption, segmentation faults, or
/// double-free errors.
///
/// # Arguments
///
/// * `ptr` - Pointer to the memory to deallocate
/// * `size` - Size of the memory in bytes (must match the original allocation)
#[wasm_bindgen]
pub unsafe fn dealloc_memory(ptr: *mut u8, size: usize) {
    // Safety: We're recreating the Vec that was forgotten in alloc_memory.
    // This is safe only if the preconditions in the safety documentation are met.
    let buffer = Vec::from_raw_parts(ptr, size, size);

    // Return the buffer to the pool for reuse
    return_buffer(buffer);
}

/// Get memory usage statistics for the buffer pool.
///
/// # Returns
///
/// A JavaScript object containing memory usage statistics
#[wasm_bindgen]
pub fn get_memory_stats() -> JsValue {
    let pool = BUFFER_POOL.lock().unwrap();
    let mut total_buffers = 0;
    let mut total_bytes = 0;

    for (size, buffers) in pool.iter() {
        total_buffers += buffers.len();
        total_bytes += size * buffers.len();
    }

    let stats = MemoryStats {
        buffer_count: total_buffers,
        bytes_allocated: total_bytes,
    };

    serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
}

// Memory statistics structure
#[derive(serde::Serialize)]
struct MemoryStats {
    buffer_count: usize,
    bytes_allocated: usize,
}
