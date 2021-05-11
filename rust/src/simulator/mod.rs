#![allow(dead_code)]

// mod screen;
// use screen::Screen;

mod window;
pub use window::WindowBuilder;
pub use window::color::Color;
use window::Window;

use std::collections::HashMap;

mod renderer;
use renderer::{Renderer, RenderMode};
pub use renderer::OriginPosition;
pub use renderer::linearalgebra::{Vector2D, Vector3D};

pub mod objects;
use objects::Object;

pub mod time;
use time::{Time, Instant, Duration};

//#region Simulator
pub struct Simulator {
    objects: HashMap<usize, Box<dyn Object>>,
    renderer: Renderer,
    use_object_clearing: bool,
    window: Window,
    pub time: Time,
    restrict_frame_rate: bool,
    frame_delay: Duration,
    last_frame_start: Instant
}

impl Simulator {
    pub fn update(&mut self) -> Result<f32, ()> {
        if !self.window.is_running() {
            return Err(());
        }

        if self.restrict_frame_rate { // enter loop to maintain framerate if needed
            std::thread::sleep(self.frame_delay - self.last_frame_start.elapsed());
        }

        self.last_frame_start = Instant::now();

        let delta = self.time.update();

        #[cfg(feature="simulator_profile")]
        let mut profile_timer = Instant::now();

        if !self.use_object_clearing {
            self.clear_screen();

            #[cfg(feature="simulator_profile")]
            println!("Frame:\n\tScreen Clear Time: {}ms", profile_timer.elapsed().as_millis());
        }

        #[cfg(feature="simulator_profile")]
        {
            profile_timer = Instant::now();
        }

        for obj in self.objects.values_mut() {
            Simulator::paint_object(obj, &self.renderer, &mut self.window, false);
            if self.use_object_clearing {
                obj.cache_transform();
            }
        }

        #[cfg(feature="simulator_profile")]
        println!("\tObject Render Time: {}ms", profile_timer.elapsed().as_millis());

        #[cfg(feature="simulator_profile")]
        {
            profile_timer = Instant::now();
        }
        
        self.window.update();

        #[cfg(feature="simulator_profile")]
        {
            println!("\tWindow Update Time: {}ms", profile_timer.elapsed().as_millis());
            println!("\tFrame time: {}ms\nEnd Frame", self.last_frame_start.elapsed().as_millis());
        }

        Ok(delta)
    }

    pub fn paint_background(&mut self) {
        self.window.fill(self.window.get_background_color());
    }

    // clears the currently rendered pixels from the screen ready to draw new ones
    pub fn clear_from_screen(&mut self, object_id: &usize) {
        #[cfg(feature="simulator_profile")]
        let clear_timer = Instant::now();

        let renderer = &self.renderer;
        let window = &mut self.window;
        let background_color = window.get_background_color();
        let object = self.objects.get_mut(&object_id).unwrap();
        let current_frame_color = object.get_frame_color();
        let current_fill_color = object.get_fill_color();
        object.set_frame_color(background_color);
        object.set_fill_color(background_color);
        Simulator::paint_object(object, renderer, window, true);
        object.set_frame_color(current_frame_color);
        object.set_fill_color(current_fill_color);

        #[cfg(feature="simulator_profile")]
        println!("\tClear Time: {}ms", clear_timer.elapsed().as_millis());
    }

    pub fn clear_screen(&mut self) {
        self.paint_background();
    }

    fn paint_object(obj: &Box<dyn Object>, renderer: &Renderer, window: &mut Window, use_cached_transform: bool) {
        let mut projected_vertexs: Vec<(i32, i32)> = Vec::new();

        let vertices = obj.get_vertices();
        let frame_color = obj.get_frame_color();

        let object_transform = if use_cached_transform {
            obj.get_cached_transform()
        }
        else {
            obj.transform()
        };

        for i in 0..vertices.len() {
            projected_vertexs.push((-1, -1));
            projected_vertexs[i] = renderer.project_to_screen(object_transform, &vertices[i].get_rel_pos(), window.get_client_size());
        }

        for i in 0..projected_vertexs.len() {
            window.draw_point(
                projected_vertexs[i].0, projected_vertexs[i].1, frame_color
            );
            for o in vertices[i].get_connections().iter() {
                window.draw_line(
                    (projected_vertexs[i].0, projected_vertexs[i].1), 
                    (
                        projected_vertexs[*o].0,
                        projected_vertexs[*o].1
                    ),
                    frame_color
                );
            }
        }
    }

    pub fn set_frame_rate_restriction(&mut self, restrict: bool) -> &mut Self {
        self.restrict_frame_rate = restrict;
        self
    }

    pub fn restrict_frame_rate(&mut self) -> &mut Self {
        self.set_frame_rate_restriction(true)
    }

    pub fn release_frame_rate(&mut self) -> &mut Self {
        self.set_frame_rate_restriction(false)
    }

    pub fn set_target_frame_rate(&mut self, target: u16) -> &mut Self {
        self.frame_delay = Duration::from_nanos(1_000_000_000 / target as u64);
        self
    }

    pub fn set_frame_rate_display(&mut self, show: bool) -> &mut Self {
        self.window.set_frame_rate_display(show);
        self
    }

    pub fn show_fps(&mut self) -> &mut Self{
        self.set_frame_rate_display(true)
    }

    pub fn hide_fps(&mut self) -> &mut Self {
        self.set_frame_rate_display(false)
    }

    pub fn enable_3d_rotation(&mut self) {
        self.renderer.enable_3d_rotation();
    }

    pub fn disable_3d_rotation(&mut self) {
        self.renderer.disable_3d_rotation();
    }

    pub fn set_3d_rotation(&mut self, toggle: bool) {
        if toggle {
            self.renderer.enable_3d_rotation();
        }
        else {
            self.renderer.disable_3d_rotation();
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) -> usize {
        let id = object.get_id();
        self.objects.insert(id, object);
        id
    }

    pub fn remove_object(&mut self, object_id: &usize) -> Option<Box<dyn Object>> {
        self.objects.remove(object_id)
    }

    pub fn get_object_by_id(&mut self, id: &usize) -> Option<&mut Box<dyn Object>> {
        self.objects.get_mut(id)
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}
//#endregion

//#region SimulationBuilder
pub struct SimulationBuilder {
    restrict_frame_rate: bool,
    use_object_clearing: bool,
    target_frame_rate: u16,
    render_mode: RenderMode,
    width: u32,
    height: u32,
    origin: OriginPosition,
    allow_3d_rotation: bool
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Self {
            restrict_frame_rate: false,
            use_object_clearing: false,
            target_frame_rate: 60,
            render_mode: RenderMode::R2D,
            width: 512,
            height: 512,
            origin: OriginPosition::MIDDLEMIDDLE,
            allow_3d_rotation: true
        }
    }

    pub fn restrict_frame_rate(mut self) -> Self {
        self.ref_restrict_frame_rate();
        self
    }

    pub fn ref_restrict_frame_rate(&mut self) -> &mut Self {
        self.restrict_frame_rate = true;
        self
    }

    pub fn release_frame_rate(mut self) -> Self {
        self.ref_release_frame_rate();
        self
    }

    pub fn ref_release_frame_rate(&mut self) -> &mut Self {
        self.restrict_frame_rate = false;
        self
    }

    pub fn set_target_frame_rate(mut self, target: u16) -> Self {
        self.ref_set_target_frame_rate(target);
        self
    }

    pub fn ref_set_target_frame_rate(&mut self, target: u16) -> &mut Self {
        self.target_frame_rate = target;
        self
    }

    pub fn lock_frame_rate(mut self, target: u16) -> Self {
        self.ref_lock_frame_rate(target);
        self
    }

    pub fn ref_lock_frame_rate(&mut self, target: u16) -> &mut Self {
        self.ref_restrict_frame_rate();
        self.ref_set_target_frame_rate(target);
        self
    }

    pub fn set_render_mode(mut self, mode: RenderMode) -> Self {
        self.ref_set_render_mode(mode);
        self
    }

    pub fn ref_set_render_mode(&mut self, mode: RenderMode) -> &mut Self {
        self.render_mode = mode;
        self
    }

    pub fn use_3d(mut self) -> Self {
        self.ref_use_3d();
        self
    }

    pub fn ref_use_3d(&mut self) -> &mut Self {
        self.render_mode = RenderMode::R3D;
        self
    }

    pub fn use_2d(mut self) -> Self {
        self.ref_use_2d();
        self
    }

    pub fn ref_use_2d(&mut self) -> &mut Self {
        self.render_mode = RenderMode::R2D;
        self
    }

    pub fn use_object_clearing(mut self) -> Self {
        self.ref_use_object_clearing();
        self
    }

    pub fn ref_use_object_clearing(&mut self) -> &mut Self {
        self.use_object_clearing = true;
        self
    }

    pub fn toggle_object_clearing(mut self) -> Self {
        self.ref_toggle_object_clearing();
        self
    }

    pub fn ref_toggle_object_clearing(&mut self) -> &mut Self {
        self.use_object_clearing = !self.use_object_clearing;
        self
    }

    pub fn use_background_fill(mut self) -> Self {
        self.ref_use_background_fill();
        self
    }

    pub fn ref_use_background_fill(&mut self) -> &mut Self {
        self.use_object_clearing = false;
        self
    }

    pub fn set_origin(mut self, origin: OriginPosition) -> Self {
        self.ref_set_origin(origin);
        self
    }

    pub fn ref_set_origin(&mut self, origin: OriginPosition) -> &mut Self {
        self.origin = origin;
        self
    }

    pub fn allow_3d_rotation(mut self) -> Self {
        self.ref_allow_3d_rotation();
        self
    }

    pub fn ref_allow_3d_rotation(&mut self) -> &mut Self {
        self.allow_3d_rotation = true;
        self
    }

    pub fn disable_3d_rotation(mut self) -> Self {
        self.ref_disable_3d_rotation();
        self
    }

    pub fn ref_disable_3d_rotation(&mut self) -> &mut Self {
        self.allow_3d_rotation = false;
        self
    }

    pub fn toggle_3d_rotation(mut self) -> Self {
        self.ref_toggle_3d_rotation();
        self
    }

    pub fn ref_toggle_3d_rotation(&mut self) -> &mut Self {
        self.allow_3d_rotation = !self.allow_3d_rotation;
        self
    }

    pub fn build(self, window_builder: WindowBuilder) -> Simulator {
        self.ref_build(window_builder)
    }

    // consume the windowbuilder used for constructing the window
    pub fn ref_build(&self, window_builder: WindowBuilder) -> Simulator {
        Simulator {
            objects: HashMap::new(),
            renderer: Renderer::new(self.render_mode, self.origin, self.allow_3d_rotation),
            use_object_clearing: self.use_object_clearing,
            time: Time::new(),
            window: window_builder.build(),
            restrict_frame_rate: self.restrict_frame_rate,
            frame_delay: Duration::from_nanos(1_000_000_000 / self.target_frame_rate as u64),
            last_frame_start: Instant::now()
        }
    }
}
//#endregion