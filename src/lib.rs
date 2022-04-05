//! freertos-sys2 provides low-level bindings to FreeRTOS functions
//!
//! NOTE: this is currently a very incomplete selection of function signatures, that match the
//! selection we needed. Additionally, the presence of some of these functions vary based on
//! FreeRTOS configuration, as does the types used.
#![no_std]
// The types here should match the naming used in FreeRTOS
#![allow(non_camel_case_types)]

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
}

pub type BaseType_t = i32;

// FIXME: these function definitions rely on TickType_t being uint32_t, freertos can be configured
// to use different tick sizes. check `FreeRTOS_config.h`. We may need to provide a feature to
// adjust this.
pub type TickType_t = u32;

pub type TaskHandle_t = *mut core::ffi::c_void;
