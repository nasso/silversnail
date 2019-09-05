const vec2 verts[4] = vec2[4](
    vec2(-1.0, -1.0),
    vec2(1.0, -1.0),
    vec2(-1.0, 1.0),
    vec2(1.0, 1.0)
);

in vec2 position_quad;

void main() {
    // gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
    gl_Position = vec4(position_quad, 0.0, 1.0);
}
