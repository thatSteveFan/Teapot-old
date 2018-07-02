extern crate glium;
use glium::glutin::*;
use glium::glutin::ControlFlow::*;

#[derive(Debug)]
pub enum Direction
{
    UP, DOWN, LEFT, RIGHT
}

#[derive(Debug)]
pub struct Rotate
{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[derive(Debug)]
pub enum EventResult
{
    Flow(ControlFlow),
    Direction(Direction),
    Rotate(Rotate)
}


pub fn handle_event(e: glium::glutin::Event, last_touch: &mut [f32; 2]) -> EventResult
{
    
    match e
    {
        
        Event::DeviceEvent{device_id:_, event:e} => handle_device_event(e),
        Event::WindowEvent{window_id:_, event: e} => handle_window_event(e, last_touch),
        Event::Awakened => EventResult::Flow(Continue),
        Event::Suspended(_) => EventResult::Flow(Continue)
    }
}

fn handle_device_event(e: DeviceEvent) -> EventResult
{
    // println!("{:?}", e);
    match e
    {
        DeviceEvent::Key(ki) => 
        {
            println!("{}", ki.scancode);
            EventResult::Flow(Break)
        },
        // DeviceEvent::MouseWheel{delta} => 
        // {
        //     EventResult::Rotate(Rotate{x:0.0, y:0.0, z: 0.1 * match delta
        //         {
        //             glium::glutin::MouseScrollDelta::LineDelta(_, y) => y,
        //             glium::glutin::MouseScrollDelta::PixelDelta(_, y) => y,
        //         }
        //     })
        // },
        _ => 
            {
                // println!("{:?}", e);
                EventResult::Flow(Continue)
            }
    }
}

fn handle_window_event(e: WindowEvent, last_touch: &mut [f32; 2]) -> EventResult
{
    match e
    {
        glium::glutin::WindowEvent::Closed => EventResult::Flow(Break),
        //this uses linux keycodes
        glium::glutin::WindowEvent::KeyboardInput{device_id: _, input} => 
        {   
            println!("{}", input.scancode);
            match input.scancode
            {
                17 => EventResult::Direction(Direction::UP),
                30 => EventResult::Direction(Direction::LEFT),
                31 => EventResult::Direction(Direction::DOWN),
                32 => EventResult::Direction(Direction::RIGHT),
                _ => EventResult::Flow(Continue)
            }
            
        },
        glium::glutin::WindowEvent::Touch(t) =>
        {
            match t.phase
            {
                TouchPhase::Started => 
                {
                    last_touch[0] = t.location.0 as f32;
                    last_touch[1] = t.location.1 as f32;
                    EventResult::Flow(Continue)
                },
                TouchPhase::Moved =>
                {
                    let y = 0.1f32 * ((t.location.0 as f32)  - (last_touch[0] as f32));
                    let x = 0.1f32 * ((t.location.1 as f32)  - (last_touch[1] as f32));
                    let z = 0.0f32;
                    let result = EventResult::Rotate(Rotate{x, y, z});
                    last_touch[0] = t.location.0 as f32;
                    last_touch[1] = t.location.1 as f32;
                    result
                },
                _ => 
                    {
                        
                        EventResult::Flow(Continue)
                    }
            }
        },
        glium::glutin::WindowEvent::MouseWheel{device_id: id, delta, phase, modifiers: mods} =>
        {
            if mods.ctrl
            {
                println!("{:?}", phase);   
                return EventResult::Flow(Continue);
            }
            EventResult::Rotate(Rotate{x:0.0, y:0.0, z: 0.1 * match delta
            {
                glium::glutin::MouseScrollDelta::LineDelta(_, y) => y,
                glium::glutin::MouseScrollDelta::PixelDelta(_, y) => y,
            }
        })
        }
        _ => EventResult::Flow(Continue)
    }
}