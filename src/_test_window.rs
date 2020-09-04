// fn main() -> Result<(), Error> {
//     println!("starting....");
//
//     let event_loop = EventLoop::new();
//
//     let mut input = WinitInputHelper::new();
//
//     let window = {
//         let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
//         WindowBuilder::new()
//             .with_title("png")
//             .with_inner_size(size)
//             .with_min_inner_size(size)
//             .build(&event_loop)
//             .unwrap()
//     };
//
//     let mut pixels = {
//         let window_size = window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(WIDTH, HEIGHT, surface_texture)?
//     };
//
//     event_loop.run(move |event, _, control_flow| {
//         if let Event::RedrawRequested(_) = event {
//             draw(pixels.get_frame());
//
//             if pixels.render().is_err() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//         }
//
//         if input.update(&event) {
//             if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//
//             if let Some(size) = input.window_resized() {
//                 pixels.resize(size.width, size.height);
//             }
//
//             window.request_redraw();
//         }
//     })
// }
//
// fn draw(frame: &mut [u8]) {
//     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
//         let rgba = if i % 2 == 0 {
//             [0x00, 0x00, 0x00, 0x00]
//         } else {
//             [0xff, 0xff, 0xff, 0x00]
//         };
//         pixel.copy_from_slice(&rgba);
//     }
// }
