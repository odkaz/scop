#version 330 core

layout (location = 0) in vec3 Position;
//layout (location = 1) in vec3 vertexColor;
//layout (location = 2) in vec2 vertexUV;

//out vec3 fragmentColor;
//out vec2 UV;

uniform mat4 mvp;

void main()
{
    gl_Position = mvp * vec4(Position, 1.0);
    //fragmentColor = vertexColor;
    //UV = vertexUV;
}
