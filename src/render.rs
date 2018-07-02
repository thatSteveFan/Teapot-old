extern crate glium;
use glium::*;
use std;
use camera;
#[path = "tuto-07-teapot.rs"]
pub mod teapot;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

static VERTEX_SHADER_SRC:&str = r#"
        #version 140 

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        out vec3 v_position;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
        }
    "#;

    static FRAGMENT_SHADER_SRC:&str = r#"
        #version 140

        in vec3 v_normal;
        in vec3 v_position;

        out vec4 color;

        uniform vec3 u_light;

        const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
        const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
        const vec3 specular_color = vec3(1.0, 1.0, 1.0);

        void main() {
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#;


pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let direction_norm = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let up_cross_direction = [up[1] * direction_norm[2] - up[2] * direction_norm[1],
             up[2] * direction_norm[0] - up[0] * direction_norm[2],
             up[0] * direction_norm[1] - up[1] * direction_norm[0]];

    let cross_norm = {
        let len = up_cross_direction[0] * up_cross_direction[0] + up_cross_direction[1] * up_cross_direction[1] + up_cross_direction[2] * up_cross_direction[2];
        let len = len.sqrt();
        [up_cross_direction[0] / len, up_cross_direction[1] / len, up_cross_direction[2] / len]
    };

    let u = [direction_norm[1] * cross_norm[2] - direction_norm[2] * cross_norm[1],
             direction_norm[2] * cross_norm[0] - direction_norm[0] * cross_norm[2],
             direction_norm[0] * cross_norm[1] - direction_norm[1] * cross_norm[0]];

    let p = [-position[0] * cross_norm[0] - position[1] * cross_norm[1] - position[2] * cross_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * direction_norm[0] - position[1] * direction_norm[1] - position[2] * direction_norm[2]];

    [
        [cross_norm[0], u[0], direction_norm[0], 0.0],
        [cross_norm[1], u[1], direction_norm[1], 0.0],
        [cross_norm[2], u[2], direction_norm[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

pub fn draw(d: &Display,  _texture: &Texture2d, camera: &camera::Camera)
{
    let positions = glium::VertexBuffer::new(d, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(d, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(d, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();

    let program = glium::Program::from_source(d, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
    let mut target = d.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.clear_depth(1.0);

    let perspective = {
        let (width, height) = target.get_dimensions();
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = std::f32::consts::FRAC_PI_3;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar)/(znear-zfar)    ,   -1.0],
            [         0.0         ,    0.0, (2.0*zfar*znear)/(znear-zfar),   0.0],
        ]
    };


    let uniforms = uniform!{
        u_light : [-0.0, 1.0, -0.4f32],
        model : [
            [0.1, 0.0, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.0],
            [0.0, 0.0, 0.1, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ],
        view: view_matrix(&camera.pos.vec, &camera.dir.forward, &camera.dir.up),
        perspective: perspective,
    };

    


    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    target.draw((&positions, &normals), &indices, &program, &uniforms,
                    &params).unwrap();

    target.finish().unwrap();

}
