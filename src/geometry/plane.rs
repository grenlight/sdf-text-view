#![allow(dead_code)]

use crate::vertex::PosTex;

pub struct Plane {
    width: f32,
    height: f32,
    h_segments: u32,
    v_segments: u32,
}

impl Plane {
    pub fn new(h_segments: u32, v_segments: u32) -> Self {
        Plane { width: 2.0, height: 2.0, h_segments: h_segments, v_segments: v_segments }
    }

    pub fn new_by_pixel(width: f32, height: f32, h_segments: u32, v_segments: u32) -> Self {
        Plane { width, height, h_segments, v_segments }
    }

    pub fn generate_vertices(&self) -> (Vec<PosTex>, Vec<u32>) {
        let segment_width = self.width / self.h_segments as f32;
        let segment_height = self.height / self.v_segments as f32;
        let h_gap = 1.0 / (self.h_segments as f32);
        let v_gap = 1.0 / (self.v_segments as f32);

        let mut vertices: Vec<PosTex> = Vec::new();

        // 从左下角开始，按列遍历
        // 下边的写法等同于 for (let h=0; h<(h_segments + 1); h++) {}
        for h in 0..=self.h_segments {
            let x: f32 = -self.half_width() + segment_width * (h as f32);
            let tex_coord_u: f32 = h_gap * (h as f32);

            for v in 0..=self.v_segments {
                let y: f32 = -self.half_height() + segment_height * (v as f32);
                let tex_coord_v: f32 = v_gap * (v as f32);
                // println!("tex_coord: {}, {} ", tex_coord_u, tex_coord_v);
                vertices.push(PosTex::vertex_f32([x, y, 0.0], [tex_coord_u, tex_coord_v]));
            }
        }

        (vertices, self.get_element_indices())
        // (vertices, self.get_line_indices())
    }

    // 返回的是线断列表，而不是线段条带
    pub fn get_line_indices(&self) -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        // 2 个线段有 3 个结点，所以需要 (self.v_segments + 1) * h
        let v_point_num = self.v_segments + 1;
        // 按列遍历
        for h in 0..=self.h_segments {
            if h == 0 {
                for v in 1..=self.v_segments {
                    let current: u32 = v;
                    // 找到同一行上个位置的索引
                    indices.push(current - 1);
                    indices.push(current);
                }
                continue;
            }
            let num: u32 = v_point_num * h;
            for v in 0..=self.v_segments {
                let current: u32 = num + v;
                // 找到上一列同一行位置的索引
                let left: u32 = current - v_point_num;
                if v == 0 {
                    indices.push(left);
                    indices.push(current);
                } else {
                    let mut lines: Vec<u32> =
                        vec![current, left, current, left - 1, current, current - 1];
                    indices.append(&mut lines);
                }
            }
        }

        indices
    }

    // 返回的是 triangle list，而不是 triangle strip
    pub fn get_element_indices(&self) -> Vec<u32> {
        let mut indices: Vec<u32> = Vec::new();
        // 2 个线段有 3 个结点，所以需要 (self.v_segments + 1) * h
        let v_point_num = self.v_segments + 1;
        // 按列遍历
        for h in 1..=self.h_segments {
            let num: u32 = v_point_num * h;
            for v in 1..(self.v_segments + 1) {
                let current: u32 = num + v;
                // 找到上一列同一行位置的索引
                let left: u32 = current - v_point_num;
                let mut lines: Vec<u32> =
                    vec![current, left, left - 1, current, left - 1, current - 1];
                indices.append(&mut lines);
            }
        }

        indices
    }

    fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}
