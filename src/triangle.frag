#version 330 core

// Interpolated values from the vertex shaders
in vec2 UV;

//in vec3 fragmentColor;
out vec3 Color;

uniform sampler2D myTextureSampler;

void main()
{
    //Color = fragmentColor;
    Color = texture(myTextureSampler, UV).rgb;
}
