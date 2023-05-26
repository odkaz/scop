#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 vertexColor;

out vec3 fragmentColor;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(Position, 1.0);
    fragmentColor = vertexColor;
}
