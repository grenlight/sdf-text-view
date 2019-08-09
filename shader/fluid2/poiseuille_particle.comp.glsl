layout (local_size_x = 1, local_size_y = 1) in;

#include fluid_layout_and_fn

layout(set = 0, binding = 1, rgba32f) uniform image2D particles;
layout (set = 0, binding = 2) buffer FluidBuffer   { FluidCell fluidCells[];    };

vec4 srcData(int u, int v) {
    return fluidCells[ v * lattice_num.x + u ].color;
}

vec4 bilinear_interpolate(vec2 uv) {
    int minX = int(floor(uv.x));
    int minY = int(floor(uv.y));
    float fx = uv.x - float(minX);
    float fy = uv.y - float(minY);
    // 插值公式： f(i+u,j+v) = (1-u)(1-v)f(i,j) + (1-u)vf(i,j+1) + u(1-v)f(i+1,j) + uvf(i+1,j+1) 
    return srcData(minX, minY) * ((1.0 - fx) * (1.0 - fy))
            + srcData(minX, minY + 1) * ((1.0 - fx) * fy)
            + srcData(minX + 1, minY) * (fx * (1.0 - fy))
            + srcData(minX + 1, minY + 1) * (fx * fy);
}

void main() {
    // 粒子个数与格子是不一致的
	ivec2 uv = ivec2(gl_GlobalInvocationID.xy);
    // uv += ivec2(1, 1);

    vec4 particle = imageLoad(particles, uv);
    
    // 计算粒子所在的 lattice
    // 粒子的坐标空间是【-1， 1】，需要转到 【0， 2】
    ivec2 ij = ivec2( (particle.xy + vec2(1.0, 1.0)) / lattice_size.x);
    // ivec2 ij = ivec2( (particle.r * 2.0) / lattice_size.x, (particle.g * 2.0) / lattice_size.y  );

    particle.xyz = particle.xyz + bilinear_interpolate(ij).xyz * lattice_size.x;
    // if (rg.x == 0.0 && rg.y == 0.0 && weight[2] == 0.0) {
    //     particle.xy = vec2(0.0, 1.0);
    // } else {
// 将格子速度转换成像素速度
    // particle.xy = particle.xy + vec2(0.01, 0.01);//imageLoad(boltzmann, ij).rg * lattice_size;
    // particle.xy = vec2(particle.x + weight[1], particle.y + weight[2]);
    // particle.xy = imageLoad(boltzmann, ij).gb;


    // }
    

    imageStore(particles, uv, particle);
}