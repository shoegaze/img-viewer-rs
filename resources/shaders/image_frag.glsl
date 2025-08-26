#version 140

in vec2 v_tex_coords;
out vec4 out_color;

uniform sampler2D tex;


void main() {
    out_color = texture(tex, v_tex_coords);
}