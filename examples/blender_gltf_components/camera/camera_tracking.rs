use bevy::prelude::*;


#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct CameraTracking{
    pub offset: Vec3
}
impl Default for CameraTracking {
    fn default() -> Self {
        CameraTracking { offset: Vec3::new(0.0, 6.0, 8.0) }
    }
}


#[derive(Component, Reflect, Default, Debug, )]
#[reflect(Component)]
pub struct CameraTrackable;

pub fn camera_track(
    mut tracking_cameras: Query<(&mut Transform, &CameraTracking), (With<Camera>, With<CameraTracking>, Without<CameraTrackable>)>,
    camera_tracked: Query<(&Transform), With<CameraTrackable>>,
) {
    
    for (mut camera_transform, tracking) in tracking_cameras.iter_mut() {
        for tracked_transform in camera_tracked.iter(){

            let target_position = tracked_transform.translation + tracking.offset;
            let eased_position = camera_transform.translation.lerp(target_position, 0.1);
            camera_transform.translation = eased_position;// + tracking.offset;// tracked_transform.translation + tracking.offset;
            *camera_transform = camera_transform.looking_at(tracked_transform.translation, Vec3::Y);
        }
    }
   
}
