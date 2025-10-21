use windows::{
    core::*,
    Win32::{Devices::Display::{GetMonitorBrightness, GetNumberOfPhysicalMonitorsFromHMONITOR, GetPhysicalMonitorsFromHMONITOR, PHYSICAL_MONITOR}, Graphics::Gdi::*, UI::WindowsAndMessaging::*},
};

fn main() -> Result<()> {
    unsafe  {
        let h_monitor = MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY);
        let mut num = 0u32;
        GetNumberOfPhysicalMonitorsFromHMONITOR(h_monitor, &mut num)?;

        let mut monitors  = vec![PHYSICAL_MONITOR::default(); num as usize];
        GetPhysicalMonitorsFromHMONITOR(h_monitor, &mut monitors)?;

        let handle = monitors[0].hPhysicalMonitor;

        let mut min = 0u32;
        let mut cur = 0u32;
        let mut max = 0u32;
        GetMonitorBrightness(handle, &mut min, &mut cur, &mut max);

        println!("Brightness: min={}, current={}, max={}", min, cur, max);
    }
    
    Ok(())
}