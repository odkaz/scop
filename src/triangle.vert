#version 330 core

layout (location = 0) in vec3 Position;

uniform mat4 mvp = mat4(
    vec4(1.0f, 0.0f, 0.0f, 0.5f),
    vec4(0.0f, 1.0f, 0.0f, 0.5f),
    vec4(0.0f, 0.0f, 1.0f, 0.0f),
    vec4(0.0f, 0.0f, 0.0f, 1.0f)
);

uniform mat4 transform;

void main()
{
    gl_Position = transform * vec4(Position, 1.0);
}
