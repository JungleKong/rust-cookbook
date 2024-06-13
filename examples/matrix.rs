use glam::{Vec4, Mat4, EulerRot};

pub fn unit_pixel_to_meter(distance: f32, fov: f32, pixel_size: isize) -> f32 {
    1.0 / (((fov.to_radians() * 0.5).tan() * distance) * 2.0 / pixel_size as f32)
}


fn main() {
    println!("unit_pixel_to_meter: {}", 2.0 * unit_pixel_to_meter(10.0, 60.0, 1440));

    // Vec4(0.32394165, 0.0, -0.003293028, 0.0), y_axis: Vec4(-0.001063694, 0.30659235, -0.10463766, 0.0), z_axis: Vec4(0.0031165031, 0.10464306, 0.30657652, 0.0), w_axis: Vec4(-1.7854137, 4.3352127, 12.41016, 1.0)
    let local_matrix = Mat4::from_cols(
        Vec4::new(0.32394165, 0.0, -0.003293028, 0.0),
        Vec4::new(-0.001063694, 0.30659235, -0.10463766, 0.0),
        Vec4::new(0.0031165031, 0.10464306, 0.30657652, 0.0),
        Vec4::new(-1.7854137, 4.3352127, 12.41016, 1.0),
    );

    let world_matrix = Mat4::from_cols(
        Vec4::new(-0.2569305, -0.0028831167, -0.033849847, 0.0),
        Vec4::new(-0.00020767242, 0.25836018, -0.020429183, 0.0),
        Vec4::new(0.03397177, -0.020225808, -0.25613326, 0.0),
        Vec4::new(-1.619475, 4.4012856, 12.798816, 1.0),
    );

    // let parent_world = Mat4::from_cols(
    //     Vec4::new(0.32394165, 0.0, -0.003293028, 0.0),
    //     Vec4::new(-0.001063694, 0.30659235, -0.10463766, 0.0),
    //     Vec4::new(0.0031165031, 0.10464306, 0.30657652, 0.0),
    //     // Vec4::new(-1.7854137, 4.3352127, 12.41016, 1.0),
    //     Vec4::new(-1.4338709, 4.121777, 12.479445, 1.0),
    // );

    let parent_world = Mat4::from_cols(
        Vec4::new(0.49997422, 0.0, -0.005082487, 0.0),
        Vec4::new(-0.001641714, 0.47319716, -0.16149864, 0.0),
        Vec4::new(0.0048100366, 0.16150698, 0.47317272, 0.0),
        // Vec4::new(-1.7854137, 4.3352127, 12.41016, 1.0),
        Vec4::new(-1.7849126, 4.3352222, 12.410182, 1.0),
    );

    let pt = parent_world.inverse() * world_matrix;
    let (scale, rotation, translation) = pt.to_scale_rotation_translation();
    println!("pt {:?}", pt);
    println!("scale {:?}", scale);
    println!("rotation {:?}", rotation.to_euler(EulerRot::XYZ));
    println!("translation {:?}", translation);
    println!("=========================");
    let pt = pt * parent_world;
    // assert!(pt.abs_diff_eq(world_matrix, 1e-6));
    println!("pt {:?}", pt);
    let (scale, rotation, translation) = pt.to_scale_rotation_translation();
    println!("scale {:?}", scale);
    println!("rotation {:?}", rotation.to_euler(EulerRot::XYZ));
    println!("translation {:?}", translation);

    let (scale, rotation, translation) = world_matrix.to_scale_rotation_translation();
    println!("scale {:?}", scale);
    println!("rotation {:?}", rotation.to_euler(EulerRot::XYZ));
    println!("translation {:?}", translation);


    println!("pt {:?}", pt);
}
// 2024-03-22T11:14:39.231+08:00 INFO  [eid::eid_data_agent] kongjy: reset_matrix parent world matrix: Mat4 { x_axis: Vec4(0.49997422, 0.0, -0.005082487, 0.0), y_axis: Vec4(-0.001641714, 0.47319716, -0.16149864, 0.0), z_axis: Vec4(0.0048100366, 0.16150698, 0.47317272, 0.0), w_axis: Vec4(-1.7849126, 4.3352222, 12.410182, 1.0) }

    // 2024-02-27T14:47:22.380+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node local matrix: Mat4 { x_axis: Vec4(-0.48780346, 0.14488965, -0.27830702, 0.0), y_axis: Vec4(0.0, 0.51445705, 0.26783195, 0.0), z_axis: Vec4(0.31376395, 0.22525749, -0.43267918, 0.0), w_axis: Vec4(-0.1, 0.6, 1.2, 1.0) }
    // 2024-02-27T14:47:22.380+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node parent matrix: Mat4 { x_axis: Vec4(0.32394165, 0.0, -0.0032930283, 0.0), y_axis: Vec4(-0.001063694, 0.30659235, -0.10463767, 0.0), z_axis: Vec4(0.0031165031, 0.10464307, 0.30657652, 0.0), w_axis: Vec4(-1.7846732, 4.121777, 12.483011, 1.0) }
    // 2024-02-27T14:47:22.380+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node world matrix: Mat4 { x_axis: Vec4(-0.15904133, 0.01529916, -0.09887696, 0.0), y_axis: Vec4(0.00028747425, 0.18575536, 0.028279401, 0.0), z_axis: Vec4(0.10005316, 0.023785342, -0.15725292, 0.0), w_axis: Vec4(-1.8139658, 4.431304, 12.78845, 1.0) }
    // 2024-02-27T14:47:32.210+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node local matrix: Mat4 { x_axis: Vec4(-0.48780346, 0.14488965, -0.27830702, 0.0), y_axis: Vec4(0.0, 0.51445705, 0.26783195, 0.0), z_axis: Vec4(0.31376395, 0.22525749, -0.43267918, 0.0), w_axis: Vec4(-0.1, 0.6, 1.2, 1.0) }
    // 2024-02-27T14:47:32.210+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node parent matrix: Mat4 { x_axis: Vec4(0.32394165, 0.0, -0.0032930283, 0.0), y_axis: Vec4(-0.001063694, 0.30659235, -0.10463767, 0.0), z_axis: Vec4(0.0031165031, 0.10464307, 0.30657652, 0.0), w_axis: Vec4(-1.4338709, 4.121777, 12.479445, 1.0) }
    // 2024-02-27T14:47:32.210+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node world matrix: Mat4 { x_axis: Vec4(-0.15904133, 0.01529916, -0.09887696, 0.0), y_axis: Vec4(0.00028747425, 0.18575536, 0.028279401, 0.0), z_axis: Vec4(0.10005316, 0.023785342, -0.15725292, 0.0), w_axis: Vec4(-1.4631635, 4.431304, 12.784884, 1.0) }
    

    // 2024-02-27T14:59:36.855+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node local matrix: Mat4 { x_axis: Vec4(-0.2569305, -0.002883147, -0.03384984, 0.0), y_axis: Vec4(-0.00020770011, 0.25836018, -0.020429222, 0.0), z_axis: Vec4(0.033971768, -0.020225821, -0.25613326, 0.0), w_axis: Vec4(-1.619475, 4.4012856, 12.798816, 1.0) }
    // 2024-02-27T14:59:36.855+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node parent matrix: Mat4 { x_axis: Vec4(0.49997422, 0.0, -0.005082487, 0.0), y_axis: Vec4(-0.001641714, 0.47319716, -0.16149864, 0.0), z_axis: Vec4(0.0048100366, 0.16150698, 0.47317272, 0.0), w_axis: Vec4(-1.7854136, 4.335215, 12.410167, 1.0) }
    // 2024-02-27T14:59:36.855+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node world matrix: Mat4 { x_axis: Vec4(-0.12861672, -0.0068312827, -0.01424535, 0.0), y_axis: Vec4(-0.0006262635, 0.11895584, -0.051390313, 0.0), z_axis: Vec4(0.015786204, -0.05093811, -0.11810149, 0.0), w_axis: Vec4(-2.5407722, 8.484989, 17.763647, 1.0) }
    // 2024-02-27T14:59:42.982+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node local matrix: Mat4 { x_axis: Vec4(-0.2569305, -0.002883147, -0.03384984, 0.0), y_axis: Vec4(-0.00020770011, 0.25836018, -0.020429222, 0.0), z_axis: Vec4(0.033971768, -0.020225821, -0.25613326, 0.0), w_axis: Vec4(-1.619475, 4.4012856, 12.798816, 1.0) }
    // 2024-02-27T14:59:42.982+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node parent matrix: Mat4 { x_axis: Vec4(0.49997422, 0.0, -0.005082487, 0.0), y_axis: Vec4(-0.001641714, 0.47319716, -0.16149864, 0.0), z_axis: Vec4(0.0048100366, 0.16150698, 0.47317272, 0.0), w_axis: Vec4(-1.4346113, 4.335215, 12.406601, 1.0) }
    // 2024-02-27T14:59:42.982+08:00 INFO  [grtdemo::app::xeid::eid_data_agent] kongjy: node world matrix: Mat4 { x_axis: Vec4(-0.12861672, -0.0068312827, -0.01424535, 0.0), y_axis: Vec4(-0.0006262635, 0.11895584, -0.051390313, 0.0), z_axis: Vec4(0.015786204, -0.05093811, -0.11810149, 0.0), w_axis: Vec4(-2.18997, 8.484989, 17.76008, 1.0) }
