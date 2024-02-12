use tauri::AppHandle;

pub async fn support_controller(handle: AppHandle) {
    // let mut gilrs = gilrs::Gilrs::new().unwrap();
    let mut controller_one_connected = false;
    let mut controller_one_connected = false;
    // loop {
    //     if !controller_one_connected || !controller_two_connected {
    //         gilrs = gilrs::Gilrs::new().unwrap();
    //     }

    //     if !controller_one_connected {
    //         let () = gilrs.gamepads().enumerate().for_each(|(id, gamepad)| {
    //             if id == 0 {
    //                 controller_one_connected = true;
    //                 handle
    //                     .emit_all("controller_one_connected", {
    //                         json!({
    //                             "connected": controller_one_connected,
    //                             "id": id,
    //                             "name": gamepad.name(),
    //                         })
    //                     })
    //                     .unwrap();
    //             }
    //         });
    //     }
    // }
}
