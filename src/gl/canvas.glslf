#version 150 core

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
    mat4 u_MVP;
};

layout (std140) uniform Force {
    float u_Force;
};

void main() {
    vec4 new_color = v_Color;
    new_color[3] = 1.0;
    Target0 = texture(t_Texture, v_Uv) * new_color;
}