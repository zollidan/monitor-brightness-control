use device_query::{DeviceEvents, DeviceEventsHandler, Keycode};
use std::{sync::{Arc, Mutex}, time::Duration}; 
use windows::{
    core::*,
    Win32::{Devices::Display::{GetMonitorBrightness, GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, SetMonitorBrightness, PHYSICAL_MONITOR}, Graphics::Gdi::*, UI::WindowsAndMessaging::*, Foundation::HANDLE},
};

const STEP: u32 = 10;

struct MonitorController {
    handle: HANDLE,
    brightness: u32,
    min: u32,
    max: u32,
}

impl MonitorController {
    fn init_monitor() -> Result<Self>{
        unsafe  {
            let h_monitor = MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY);
            let mut num = 0u32;
            GetNumberOfPhysicalMonitorsFromHMONITOR(h_monitor, &mut num)?;

            let mut monitors  = vec![PHYSICAL_MONITOR::default(); num as usize];
            GetPhysicalMonitorsFromHMONITOR(h_monitor, &mut monitors)?;

            let handle = monitors[0].hPhysicalMonitor;
            
            let mut min = 0u32;
            let mut current = 0u32;
            let mut max = 0u32;
            GetMonitorBrightness(handle, &mut min, &mut current, &mut max);

            Ok(Self {
                handle,
                brightness: current,
                min,
                max,
            })
        }
    }

    fn apply(&self) {
        unsafe {
            SetMonitorBrightness(self.handle, self.brightness);
        }
    }

    fn increase(&mut self) {
        self.brightness = self.brightness.saturating_add(STEP).min(self.max);
        self.apply();
    }

    fn reduce(&mut self) {
        self.brightness = self.brightness.saturating_sub(STEP).max(self.min);
        self.apply();
    }

    fn get_brightness(&self) -> u32 {
        self.brightness
    }
}

unsafe impl Send for MonitorController {}
unsafe impl Sync for MonitorController {}

fn main() -> Result<()> {

    let controller = Arc::new(Mutex::new(MonitorController::init_monitor()?));


    let device_state = DeviceEventsHandler::new(Duration::from_millis(10))
    .expect("Failed to start event loop");

    let ctrl_clone = Arc::clone(&controller);
    let _guard = device_state.on_key_down(move |key| {
        
        let mut ctrl = ctrl_clone.lock().unwrap();  

        if key == &Keycode::A {
            ctrl.reduce();
            println!("{}", ctrl.get_brightness())
            
        }else if key == &Keycode::D {
            ctrl.increase();
            println!("{}", ctrl.get_brightness())
        }
    });

    loop {}
}