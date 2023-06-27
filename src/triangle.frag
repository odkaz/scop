#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

// texture sampler
uniform sampler2D texture1;
uniform vec3 objectColor;
uniform vec3 lightColor;
//uniform vec3 lightPos;

void main()
{
	//FragColor = texture(texture1, TexCoord);

	float ambientStrength = 0.3;
	vec3 ambient = ambientStrength * lightColor;

	vec3 lightPos = vec3(2.4, 4.0, 4.0);
	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPos - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);
	vec3 diffuse = diff * lightColor;

	vec3 result = (ambient + diffuse) * objectColor;
	FragColor = vec4(result, 1.0);
}
