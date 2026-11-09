// src/app/queue.rs

use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};

use crate::hal::*;

pub const APP_QUEUE_ORIGIN_TIMER: u32 = 1;
pub const APP_QUEUE_ORIGIN_PULSE: u32 = 2;

const APP_QUEUE_SIZE: u32 = 16;
const APP_QUEUE_BYTE_MASK: u32 = 0xFF;
const APP_QUEUE_ORIGIN_SHIFT: u32 = 8;

const APP_QUEUE_SLOT_INIT: AtomicU16 = AtomicU16::new(0);

static APP_QUEUE_SLOTS: [AtomicU16; APP_QUEUE_SIZE as usize] =
    [APP_QUEUE_SLOT_INIT; APP_QUEUE_SIZE as usize];
static APP_QUEUE_HEAD: AtomicU32 = AtomicU32::new(0);
static APP_QUEUE_TAIL: AtomicU32 = AtomicU32::new(0);

pub struct AppQueueItem {
    pub origin: u32,
    pub data: u32,
}

fn app_queue_next(index: u32) -> u32 {
    (index + 1) % APP_QUEUE_SIZE
}

fn app_queue_pack(origin: u32, data: u32) -> u16 {
    (((origin & APP_QUEUE_BYTE_MASK) << APP_QUEUE_ORIGIN_SHIFT)
        | (data & APP_QUEUE_BYTE_MASK)) as u16
}

fn app_queue_unpack(word: u16) -> AppQueueItem {
    let value = word as u32;
    AppQueueItem {
        origin: value >> APP_QUEUE_ORIGIN_SHIFT,
        data: value & APP_QUEUE_BYTE_MASK,
    }
}

fn app_queue_pop_locked() -> Option<AppQueueItem> {
    let tail = APP_QUEUE_TAIL.load(Ordering::Acquire);
    if tail == APP_QUEUE_HEAD.load(Ordering::Acquire) {
        return None;
    }
    let word = APP_QUEUE_SLOTS[tail as usize]
        .load(Ordering::Relaxed);
    APP_QUEUE_TAIL.store(app_queue_next(tail), Ordering::Release);
    Some(app_queue_unpack(word))
}

pub fn app_queue_reset() {
    let state = hal_boot_critical_enter();
    APP_QUEUE_HEAD.store(0, Ordering::Release);
    APP_QUEUE_TAIL.store(0, Ordering::Release);
    hal_boot_critical_exit(state);
}

pub fn app_queue_push_isr(origin: u32, data: u32) -> bool {
    let head = APP_QUEUE_HEAD.load(Ordering::Acquire);
    let next = app_queue_next(head);
    if next == APP_QUEUE_TAIL.load(Ordering::Acquire) {
        return false;
    }
    APP_QUEUE_SLOTS[head as usize]
        .store(app_queue_pack(origin, data), Ordering::Relaxed);
    APP_QUEUE_HEAD.store(next, Ordering::Release);
    true
}

pub fn app_queue_pop() -> Option<AppQueueItem> {
    let state = hal_boot_critical_enter();
    let item = app_queue_pop_locked();
    hal_boot_critical_exit(state);
    item
}