use nalgebra_glm::{self as glm, Mat4, Vec3};

pub struct Camera {
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
}

pub enum CameraType {
    Perspective {
        fov: f32,
        aspect: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    },
}

pub struct CameraCreateInfo {
    pub position: Vec3,
    pub camera_type: CameraType,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(view_matrix: Mat4, projection_matrix: Mat4) -> Self {
        Camera {
            view_matrix,
            projection_matrix,
        }
    }
}

impl From<CameraCreateInfo> for Camera {
    fn from(create_info: CameraCreateInfo) -> Self {
        let CameraCreateInfo {
            position,
            camera_type,
            near,
            far,
        } = create_info;

        let view_matrix = glm::translate(&Mat4::identity(), &-position);

        let projection_matrix = match camera_type {
            CameraType::Perspective { fov, aspect } => {
                let w = (fov.to_radians() / 2.0).tan();
                let h = w / aspect;
                let fovy = 2.0 * h.atan();

                glm::perspective(aspect, fovy, near, far)
            }
            CameraType::Orthographic {
                left,
                right,
                bottom,
                top,
            } => glm::ortho(left, right, bottom, top, near, far),
        };

        Camera {
            view_matrix,
            projection_matrix,
        }
    }
}
