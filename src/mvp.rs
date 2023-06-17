extern crate nalgebra_glm as glm;

pub fn mvp() -> glm::Mat4{

    //window
    let trans = glm::identity();
    let trans = glm::translate(&trans, &glm::vec3(-0.5, 0.5, 0.0)); //translate
    // let trans = glm::rotate(
    //     &trans,
    //     // glm::radians(&glm::vec1(0.0))[0],
    //     (currentTime as f32) / 1000.0,
    //     &glm::vec3(0.0, 1.0, 0.0),
    // );
    // let trans = glm::scale(&trans, &glm::vec3(1.0, 1.0, 1.0)); //scale
    // let Projection = glm::perspective(
    //     glm::radians(&glm::vec1(45.0))[0],
    //     width as f32 / height as f32,
    //     0.1,
    //     100.0,
    // );
    // let cam_pos = glm::vec3(4.0, 3.0, 3.0);
    // let origin = glm::vec3(0.0, 0.0, 0.0);
    // let View = glm::look_at(
    //     // &glm::vec3(4.0, 3.0, 3.0), // Camera is at (4,3,3), in World Space
    //     &cam_pos,
    //     // &(cam_pos + direction), // and looks at the origin
    //     &origin,
    //     &glm::vec3(0.0, 1.0, 0.0), // Head is up (set to 0,-1,0 to look upside-down)
    // );
    let model = trans;
    // let mvp = Projection * View * model;
    let mvp = model;
    return mvp;
}
