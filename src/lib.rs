//! freertos-sys2 provides low-level bindings to FreeRTOS functions
//!
//! NOTE: this is currently a very incomplete selection of function signatures, that match the
//! selection we needed. Additionally, the presence of some of these functions vary based on
//! FreeRTOS configuration, as does the types used.
#![no_std]
// The types, values, functions, and other items should match the naming used in FreeRTOS
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

extern "C" {
    pub fn xTaskGetTickCountFromISR() -> TickType_t;
    pub fn xTaskGetTickCount() -> TickType_t;

    pub fn xTaskGetCurrentTaskHandle() -> TaskHandle_t;

    pub fn vTaskResume(xTaskToResume: TaskHandle_t);
    pub fn vTaskSuspend(xTaskToSuspend: TaskHandle_t);
    pub fn vTaskDelayUntil(pxPreviousWakeTime: *mut TickType_t, xTimeIncrement: TickType_t);
    pub fn xTaskDelayUntil(
        pxPreviousWakeTime: *mut TickType_t,
        xTimeIncrement: TickType_t,
    ) -> BaseType_t;
    pub fn vTaskDelay(xTicksToDelay: TickType_t);

    pub fn pvTaskGetThreadLocalStoragePointer(
        xTaskToQuery: TaskHandle_t,
        xIndex: BaseType_t,
    ) -> *mut c_void;

    pub fn vTaskSetThreadLocalStoragePointer(
        xTaskToSet: TaskHandle_t,
        xIndex: BaseType_t,
        pvValue: *mut c_void,
    );

    pub fn xQueueGenericReceive(
        xQueue: QueueHandle_t,
        pvBuffer: *const c_void,
        xTicksToWait: TickType_t,
        xJustPeek: BaseType_t,
    ) -> BaseType_t;

    pub fn xQueueGenericSend(
        xQueue: QueueHandle_t,
        pvItemToQueue: *const c_void,
        xTicksToWait: TickType_t,
        xCopyPosition: BaseType_t,
    ) -> BaseType_t;

    /*
    pub fn xQueueGenericCreateStatic(
        uxQueueLength: UBaseType_t,
        uxItemSize: UBaseType_t,
        pucQueueStorage: *mut u8,
        pxStaticQueue: *mut StaticQueue_t,
        ucQueueType: u8,
    ) -> QueueHandle_t;
    */

    pub fn xQueueCreateMutex(ucQueueType: u8) -> QueueHandle_t;

    pub fn vQueueDelete(xQueue: QueueHandle_t);

    pub fn pvPortMalloc(xWantedSize: usize) -> *mut c_void;
    pub fn vPortFree(pv: *mut c_void);
}

pub type BaseType_t = i32;
pub type UBaseType_t = u32;

// FIXME: these function definitions rely on TickType_t being uint32_t, freertos can be configured
// to use different tick sizes. check `FreeRTOS_config.h`. We may need to provide a feature to
// adjust this.
pub type TickType_t = u32;

pub type TaskHandle_t = *mut c_void;
pub type QueueHandle_t = *mut c_void;
pub type SemaphoreHandle_t = *mut c_void;

// NOTE: items below here are defined as macros in freertos9, and should be scrutinized if one makes
// changes to freertos

pub const pdFALSE: BaseType_t = 0;
pub const pdTRUE: BaseType_t = 1;

pub const queueSEND_TO_BACK: BaseType_t = 0;
pub const queueSEND_TO_FRONT: BaseType_t = 1;
pub const queueOVERWRITE: BaseType_t = 2;

pub const semGIVE_BLOCK_TIME: TickType_t = 0;

pub const queueQUEUE_TYPE_BASE: u8 = 0;
pub const queueQUEUE_TYPE_SET: u8 = 0;
pub const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;

pub const portMAX_DELAY: TickType_t = TickType_t::MAX;

pub unsafe fn xSemaphoreTake(xSemaphore: SemaphoreHandle_t, xBlockTime: TickType_t) -> BaseType_t {
    unsafe { xQueueGenericReceive(xSemaphore, core::ptr::null(), xBlockTime, pdFALSE) }
}

pub unsafe fn xSemaphoreGive(xSemaphore: SemaphoreHandle_t) -> BaseType_t {
    unsafe {
        xQueueGenericSend(
            xSemaphore,
            core::ptr::null(),
            semGIVE_BLOCK_TIME,
            queueSEND_TO_BACK,
        )
    }
}

pub unsafe fn xSemaphoreCreateMutex() -> SemaphoreHandle_t {
    unsafe { xQueueCreateMutex(queueQUEUE_TYPE_MUTEX) }
}

pub unsafe fn vSemaphoreDelete(xSemaphore: SemaphoreHandle_t) {
    unsafe { vQueueDelete(xSemaphore) }
}
